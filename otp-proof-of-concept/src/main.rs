/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::crc::{Crc, InputReverseConfig};
use {defmt_rtt as _, panic_probe as _};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OtpDeviceInfo {
    pub dev_sn: [u8; 12],
    pub crc: [u8; 4],
}

impl OtpDeviceInfo {
    pub fn is_ascii_digit(&self) -> bool {
        let mut initial_space_end = false;
        for char in self.dev_sn {
            if char == b' ' {
                if initial_space_end {
                    return false;
                }
            } else if b'0' <= char && char <= b'9' {
                initial_space_end = true;
            } else {
                return false;
            }
        }
        true
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    // InputReverseConfig::Halfword
    let Ok(crc_config) = embassy_stm32::crc::Config::new(InputReverseConfig::Word, false, 0xA097)
    else {
        panic!("Something went horribly wrong")
    };
    let mut crc = Crc::new(p.CRC, crc_config);

    let otp_space: &OtpDeviceInfo = unsafe { core::mem::transmute(0x1FFF_7000) };

    if otp_space.is_ascii_digit() == false {
        defmt::error!("Wrong OTP partition")
    } else {
        crc.reset();
        let checksum = crc.feed_bytes(&otp_space.dev_sn);
        let otp_checksum_expected = checksum.to_ne_bytes();
        let otp_checksum_actual = otp_space.crc;
        defmt::info!(
            "flash crc expected 0x{:02X}, actual 0x{:02X}",
            otp_checksum_expected,
            otp_checksum_actual,
        );
        defmt::info!("flash_space {:?}", otp_space.crc);

        if otp_checksum_actual == otp_checksum_expected {
            defmt::info!("SerialNumber : {=[u8]:a}", otp_space.dev_sn);

            defmt::info!("HAPPY! It works!");
        } else {
            defmt::error!("No CRC has problem");
        }
    }
}
