/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

mod config;

use bit_field::BitField;
use std::time::Duration;

use clap::Parser;
use probe_rs::{
    architecture::arm::ArmError,
    flashing::{self, DownloadOptions, FlashLoader},
    Error, MemoryInterface, Permissions, Session,
};

#[allow(non_upper_case_globals)]
mod app_custom {
    pub(crate) const address: u64 = 0x0800F800;
}

#[allow(non_upper_case_globals)]
mod flash_optr {
    pub(crate) const address: u64 = 0x4002_2020;
}

#[allow(non_upper_case_globals)]
mod flash_cr {
    pub(crate) const address: u64 = 0x4002_2014;
    pub(crate) mod bits {
        pub(crate) const optstrt: usize = 17;
        pub(crate) const obl_launch: usize = 27;
        pub(crate) const lock: usize = 31;
        pub(crate) const optlock: usize = 30;
    }
}

#[allow(non_upper_case_globals)]
mod flash_sr {
    pub(crate) const address: u64 = 0x4002_2010;
    pub(crate) mod bits {
        pub(crate) const bsy1: usize = 16;
    }
}

#[allow(non_upper_case_globals)]
mod flash_keyr {
    pub(crate) const address: u64 = 0x4002_2008;

    pub(crate) const key1: u32 = 0x45670123;
    pub(crate) const key2: u32 = 0xCDEF89AB;
}

#[allow(non_upper_case_globals)]
mod flash_optkeyr {
    pub(crate) const address: u64 = 0x4002_200C;

    pub(crate) const optkey1: u32 = 0x08192A3B;
    pub(crate) const optkey2: u32 = 0x4C5D6E7F;
}

fn get_rdp(session: &mut Session) -> Result<u8, anyhow::Error> {
    let mut core = session.core(0)?;

    Ok(core.read_word_8(flash_optr::address)?)
}

fn set_rdp(session: &mut Session, rdp: u8) -> Result<(), anyhow::Error> {
    let mut core = session.core(0)?;

    println!("Starting writie RDP 0x{:02X}", rdp);

    core.reset()?;
    println!("Initial MCU reset");

    let flash_optr = core.read_word_32(flash_optr::address)?;

    let cr = core.read_word_32(flash_cr::address)?;
    if cr.get_bits(flash_cr::bits::optlock..=flash_cr::bits::lock) != 0 {
        println!("OPTLOCK in FLASH_CR is locked : 0x{:08X}", cr);
    }
    // Unlocking Flash memory, to unlock `FLASH_CR`
    core.write_word_32(flash_keyr::address, flash_keyr::key1)?; // 0x45670123
    core.write_word_32(flash_keyr::address, flash_keyr::key2)?;

    println!("Wrote KEY1, KEY2 on FLASH_KEYR");

    // Double check LOCK
    let cr = core.read_word_32(flash_cr::address)?;
    if cr.get_bit(flash_cr::bits::lock) {
        println!("[31b]LOCK in FLASH_CR is still locked : 0x{:08X}", cr);
        return Err(anyhow::format_err!("[31b]LOCK"));
    }

    // Allow editing optlock
    core.write_word_32(flash_optkeyr::address, flash_optkeyr::optkey1)?; // 0x08192A3B
    core.write_word_32(flash_optkeyr::address, flash_optkeyr::optkey2)?;

    println!("Wrote OPTKEY1, OPTKEY2 on OPTKEYR");
    core.write_word_32(flash_cr::address, 0x0000_0000)?;

    // Double check OPTLOCK
    let cr = core.read_word_32(flash_cr::address)?;
    if cr.get_bit(flash_cr::bits::optlock) {
        println!("[30b]OPTLOCK in FLASH_CR is still locked : 0x{:08X}", cr);
        return Err(anyhow::format_err!("[30b]OPTLOCK"));
    }

    // Access to FLASH_OPTR to change RDP L0 from L1 (L2 impossible)
    let new_rdp = (flash_optr & 0xFFFF_FF00) | (rdp as u32);
    core.write_word_32(flash_optr::address, new_rdp)?;
    println!("Wrote new RDP 0x{:02X} on FLASH_OPTR", rdp);

    // Check busy bit (BSY1:b16)
    for _ in 0..10 {
        let sr = core.read_word_32(flash_sr::address)?;
        if !sr.get_bit(flash_sr::bits::bsy1) {
            break;
        } else {
            std::thread::sleep(Duration::from_millis(10));
            println!("Wait BSY1 bit clearing : 0x{:08X}", sr);
        }
    }

    // OPTSTRT(b17) set
    core.write_word_32(flash_cr::address, 1 << flash_cr::bits::optstrt)?;

    for _ in 0..10 {
        let sr = core.read_word_32(flash_sr::address)?;
        let cr = core.read_word_32(flash_cr::address)?;

        if !sr.get_bit(flash_sr::bits::bsy1) && !cr.get_bit(flash_cr::bits::optlock) {
            break;
        } else {
            std::thread::sleep(Duration::from_millis(10));
            println!("Wait SR 0x{:08X}, CR 0x{:08X}", sr, cr);
        }
    }

    // Set OBL_LAUNCH again
    // RM 0454 3.4.2 FLASH option byte programming - Option byte loading
    // Option byte loader performs a read of the options block and stores
    // the data into internal option registers.
    // These internal registers configure the system and can be read by software.
    // Setting OBL_LAUNCH generates a reset so the option byte loading
    // is performed under System reset.
    match core.write_word_32(flash_cr::address, 1 << flash_cr::bits::obl_launch) {
        Ok(_) => {
            // After bit-setting OBL_LAUNCH, the MCU will hard reset.
            // probe-rs will forcefully terminate the session at this time,
            // resulting in the intended error. If it is Ok(_),
            // something is different or the behavior is unintended.
            println!("Ok but not Ok on OBL_LAUNCH");
        }

        Err(probe_rs::Error::Arm(ArmError::Probe(probe_rs::DebugProbeError::ProbeSpecific(_)))) => {
            // STLINK-v3 get Arm(Probe(ProbeSpecific(CommandFailed(SwdDpError))))
            // ANYWAY, this case is correct case
        }
        Err(e) => {
            println!("Is this error right for OBL_LAUNCH? {:?}", e);
        }
    }

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    // Attach to a chip.
    let prev = {
        let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;

        let prev = get_rdp(&mut session)?;

        if prev == 0xAA {
            set_rdp(&mut session, 0xBB)?;
        } else {
            set_rdp(&mut session, 0xAA)?;
        }
        drop(session);
        prev
    };

    let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;
    let mut loader = session.target().flash_loader();

    let _ = loader.add_data(app_custom::address, &[0x1, 0x2, 0x3, 0x4]);
    let _ = loader.commit(&mut session, DownloadOptions::default());
    let next = get_rdp(&mut session)?;

    println!("prev : {:2X} ---> next : {:2X}", prev, next);

    println!(
        "try write : {:X}",
        session
            .core(0)?
            .read_word_32(app_custom::address)
            .unwrap_or(0xFFFF_FFF1)
    );

    Ok(())
}
