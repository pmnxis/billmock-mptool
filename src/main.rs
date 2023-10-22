/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

mod config;
mod programmer;

use billmock_otp_dev_info::*;
use bit_field::BitField;
use std::time::Duration;

use clap::Parser;
use probe_rs::{
    flashing::{self, DownloadOptions, FlashLoader},
    Error, MemoryInterface, Permissions, Session,
};

fn main() -> Result<(), anyhow::Error> {
    // Attach to a chip.
    let rdp: Result<u8, anyhow::Error> = {
        let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;

        let prev = programmer::get_rdp(&mut session)?;

        if prev == 0xAA {
            println!("Skip writing RDP");
        } else {
            programmer::set_rdp(&mut session, 0xAA)?;
            println!("prev : {:2X} ---> next : {:2X}", prev, 0xAA);
        }
        drop(session);

        Ok(0xAAu8)
    };

    if rdp.is_ok() {
        let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;
        let mut _loader = session.target().flash_loader();

        if let Ok(raw_otp) = programmer::get_otp(&mut session) {
            let otp = OtpDeviceInfo::from_u64_arr(&raw_otp);
            match otp.check_and_sn_u64() {
                Ok(sn) => {
                    // report server, this is second time
                    println!("Already have serial number : {:12}", sn);
                }
                Err(OtpDeviceInfoParseErorr::NotCarved) => {
                    // get new serial number from server
                    let new_sn = 90123456;
                    let otp_u64_arr = OtpDeviceInfo::new(new_sn).to_u64_arr();
                    programmer::set_otp(&mut session, otp_u64_arr)?;

                    println!("New board detected, write to : {:12}", new_sn);
                }
                Err(e) => {
                    println!("Serial number has problem {:?}", e);
                }
            }
        }
    }

    Ok(())
}
