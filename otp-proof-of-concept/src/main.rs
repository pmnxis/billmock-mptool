/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use billmock_otp_dev_info::OtpDeviceInfo;
use embassy_executor::Spawner;
use embassy_stm32::crc::{Crc, InputReverseConfig};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    // InputReverseConfig::Halfword
    let Ok(crc_config) = embassy_stm32::crc::Config::new(InputReverseConfig::Word, false, 0xA097)
    else {
        panic!("Something went horribly wrong")
    };
    let mut crc = Crc::new(p.CRC, crc_config);

    let otp_space = OtpDeviceInfo::from_stm32g0();

    match otp_space.check_and_sn() {
        Err(e) => {
            defmt::error!("Wrong OTP partition : {:?}", e);
        }
        Ok(()) => {
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
}
