/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use bit_field::BitField;
use probe_rs::{
    architecture::arm::ArmError,
    // Error,
    MemoryInterface,
    // Permissions,
    Session,
};
use std::time::Duration;

// #[allow(non_upper_case_globals)]
// mod app_custom {
//     pub(crate) const address: u64 = 0x0800_F800;
// }

#[allow(non_upper_case_globals)]
mod flash_optr {
    pub(crate) const address: u64 = 0x4002_2020;
}

#[allow(non_upper_case_globals)]
mod flash_cr {
    pub(crate) const address: u64 = 0x4002_2014;
    pub(crate) mod bits {
        /// PG: Flash memory programming enable
        pub(crate) const pg: usize = 0;
        /// OPTSTRT: Start of modification of option bytes
        pub(crate) const optstrt: usize = 17;
        /// OBL_LAUNCH: Option byte load launch
        pub(crate) const obl_launch: usize = 27;
        /// OPTLOCK: Options Lock
        pub(crate) const lock: usize = 31;
        /// LOCK: FLASH_CR Lock
        pub(crate) const optlock: usize = 30;
    }
}

#[allow(non_upper_case_globals)]
mod flash_sr {
    pub(crate) const address: u64 = 0x4002_2010;
    pub(crate) mod bits {
        /// EOP: End of operation
        pub(crate) const eop: usize = 0;

        /// PROGERR: Programming error
        pub(crate) const progerr: usize = 3;

        /// PGSERR: Programming sequence error
        pub(crate) const pgserr: usize = 7;

        /// BSY1: Busy
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

#[allow(non_upper_case_globals)]
mod otp_area {
    pub(crate) const address: u64 = 0x1FFF_7000;
}

pub fn get_rdp(session: &mut Session) -> Result<u8, anyhow::Error> {
    let mut core = session.core(0)?;

    Ok(core.read_word_8(flash_optr::address)?)
}

pub fn set_rdp(session: &mut Session, rdp: u8) -> Result<(), anyhow::Error> {
    let mut core = session.core(0)?;

    println!("Starting write RDP 0x{:02X}", rdp);

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

        Err(probe_rs::Error::Arm(ArmError::Probe(
            probe_rs::probe::DebugProbeError::ProbeSpecific(_),
        ))) => {
            // STLINK-v3 get Arm(Probe(ProbeSpecific(CommandFailed(SwdDpError))))
            // ANYWAY, this case is correct case
        }
        Err(e) => {
            println!("Is this error right for OBL_LAUNCH? {:?}", e);
        }
    }

    Ok(())
}

pub fn get_otp(session: &mut Session) -> Result<[u64; 2], anyhow::Error> {
    let mut ret = [0u64; 2];
    session.core(0)?.read_64(otp_area::address, &mut ret)?;

    Ok(ret)
}

pub fn set_otp(session: &mut Session, otp_u64_arr: [u64; 2]) -> Result<(), anyhow::Error> {
    let mut prev_otp_area = [0u8; 16];
    let mut next_otp_area = [0u8; 16];
    let mut core = session.core(0)?;

    println!("Starting writie OTP. {:8X?}", otp_u64_arr);

    core.read_8(otp_area::address, &mut prev_otp_area)?;

    core.reset_and_halt(Duration::from_millis(100))?;
    println!("Initial MCU reset");

    // Previous unlocking flash memory
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

    // Check busy bit (BSY1:b16)
    // 1. Check that no Main Flash memory operation is ongoing by checking
    // the BSY1 bit of the FLASH status register (FLASH_SR).
    // 2. Check and clear all error programming flags due to a previous programming.
    // If not, PGSERR is set.
    for _ in 0..10 {
        let sr = core.read_word_32(flash_sr::address)?;
        if !sr.get_bit(flash_sr::bits::bsy1) && !sr.get_bit(flash_sr::bits::pgserr) {
            break;
        } else {
            std::thread::sleep(Duration::from_millis(10));
            println!("Wait BSY1, PGSERR bit clearing : 0x{:08X}", sr);
            core.reset_and_halt(Duration::from_millis(100))?;
        }
    }

    // 3. Set the PG bit of the FLASH control register (FLASH_CR).
    let mut cr = core.read_word_32(flash_cr::address)?;
    cr.set_bit(flash_cr::bits::pg, true);
    core.write_word_32(flash_cr::address, cr)?;
    println!("Write on CR : 0x{:08X}", cr);

    // 4. Perform the data write operation at the desired memory address,
    // inside Main memory block or OTP area. Only double word (64 bits)
    // can be programmed.
    core.write_64(otp_area::address, &otp_u64_arr)?;

    // 5. Wait until the BSY1 bit of the FLASH status register (FLASH_SR) is cleared.
    for i in 0..4 {
        let sr = core.read_word_32(flash_sr::address)?;
        if !sr.get_bit(flash_sr::bits::bsy1) {
            break;
        } else {
            std::thread::sleep(Duration::from_millis(25));
            if i == 0 {
                println!("Wait BSY1 bit clearing : 0x{:08X}", sr);
            }
        }
    }
    let sr = core.read_word_32(flash_sr::address)?;

    if sr.get_bit(flash_sr::bits::progerr) && sr.get_bit(flash_sr::bits::progerr) {
        println!("OTP is already worn-out");
        return Err(anyhow::format_err!("OTP-Worn-Out"));
    }

    // 6. Check that EOP flag of the FLASH status register (FLASH_SR)
    // is set (programming operation succeeded), and clear it by software.
    for i in 0..10 {
        let mut sr = core.read_word_32(flash_sr::address)?;
        if sr.get_bit(flash_sr::bits::eop) {
            sr.set_bit(flash_sr::bits::eop, false);
            core.write_word_32(flash_sr::address, sr)?;

            break;
        } else {
            std::thread::sleep(Duration::from_millis(80));
            if i == 0 {
                println!("Wait EOP bit setted : 0x{:08X}", sr);
            }
        }
    }

    // 7. Clear the PG bit of the FLASH control register (FLASH_CR)
    // if there no more programming request anymore.
    let mut cr = core.read_word_32(flash_cr::address)?;
    cr.set_bit(flash_cr::bits::pg, false);
    core.write_word_32(flash_cr::address, cr)?;
    println!("Write on CR : 0x{:08X}", cr);

    core.read_8(otp_area::address, &mut next_otp_area)?;

    println!(
        "OTP area : {:02X?} ---> {:02X?}",
        prev_otp_area, next_otp_area
    );

    Ok(())
}
