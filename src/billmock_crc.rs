/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use crc::{Algorithm, Crc};

const BILLMOCK_CRC: Algorithm<u32> = Algorithm {
    width: 32,
    poly: 0x4C11DB7,
    init: 0xA097,
    refin: true,
    refout: false,
    xorout: 0x0000,
    check: 0,
    residue: 0x0000,
};

pub fn billmock_crc(bytes: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&BILLMOCK_CRC);
    let mut digest = crc.digest();
    digest.update(&bytes);
    digest.finalize()
}

pub fn otp_new(sn: u64) -> Result<[u8; 16], ()> {
    let s = format!("{:12}", sn);
    let bytes = s.as_bytes();
    if bytes.len() != 12 {
        return Err(());
    }

    let mut buffer = [0u8; 16];
    buffer[0..12].copy_from_slice(bytes);

    let checksum = billmock_crc(&buffer[0..12]);
    buffer[12..16].copy_from_slice(&checksum.to_le_bytes());

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_test() {
        //! ```rs
        //! const STUFF: [&[u8; 12]; 10] = [
        //!     b"    99991111",
        //!     b"B@32rds236Td",
        //!     b"212824350-78",
        //!     b"    11010100",
        //!     b"  dks9dj290u",
        //!     b" v82d092nov0",
        //!     b"ijv03nif0h8w",
        //!     b" s d d d dd2",
        //!     b"    97372793",
        //!     b"    22232456",
        //! ];
        //!
        //!
        //! #[embassy_executor::main]
        //! async fn main(_spawner: Spawner) {
        //!     let p = embassy_stm32::init(embassy_stm32::Config::default());
        //!
        //!     // InputReverseConfig::Halfword
        //!     let Ok(crc_config) = embassy_stm32::crc::Config::new(InputReverseConfig::Word, false, 0xA097)
        //!     else {
        //!         panic!("Something went horribly wrong")
        //!     };
        //!     let mut crc = Crc::new(p.CRC, crc_config);
        //!
        //!     for a in STUFF {
        //!         crc.reset();
        //!         let result = crc.feed_bytes(a);
        //!         defmt::println!(
        //!             "assert_eq!((billmock_crc(&{:#X}), 0x{:08X});",
        //!             a,
        //!             result,
        //!             result.to_be_bytes(),
        //!             result.to_ne_bytes(),
        //!         );
        //!     }
        //! }
        //! ```
        assert_eq!(
            billmock_crc(&[0x20, 0x20, 0x20, 0x20, 0x39, 0x39, 0x39, 0x39, 0x31, 0x31, 0x31, 0x31]),
            0x6058BC8F
        );
        assert_eq!(
            billmock_crc(&[0x42, 0x40, 0x33, 0x32, 0x72, 0x64, 0x73, 0x32, 0x33, 0x36, 0x54, 0x64]),
            0xAF212016
        );
        assert_eq!(
            billmock_crc(&[0x32, 0x31, 0x32, 0x38, 0x32, 0x34, 0x33, 0x35, 0x30, 0x2D, 0x37, 0x38]),
            0x0E4AAF09
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x20, 0x20, 0x20, 0x31, 0x31, 0x30, 0x31, 0x30, 0x31, 0x30, 0x30]),
            0xF6CC170B
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x20, 0x64, 0x6B, 0x73, 0x39, 0x64, 0x6A, 0x32, 0x39, 0x30, 0x75]),
            0xF1589A9C
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x76, 0x38, 0x32, 0x64, 0x30, 0x39, 0x32, 0x6E, 0x6F, 0x76, 0x30]),
            0xA2838663
        );
        assert_eq!(
            billmock_crc(&[0x69, 0x6A, 0x76, 0x30, 0x33, 0x6E, 0x69, 0x66, 0x30, 0x68, 0x38, 0x77]),
            0x1B2B9024
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x73, 0x20, 0x64, 0x20, 0x64, 0x20, 0x64, 0x20, 0x64, 0x64, 0x32]),
            0x3EB42551
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x20, 0x20, 0x20, 0x39, 0x37, 0x33, 0x37, 0x32, 0x37, 0x39, 0x33]),
            0x242362F1
        );
        assert_eq!(
            billmock_crc(&[0x20, 0x20, 0x20, 0x20, 0x32, 0x32, 0x32, 0x33, 0x32, 0x34, 0x35, 0x36]),
            0x56E18A06
        );
    }

    #[test]
    fn make_otp_area() {
        assert_eq!(
            Ok([
                0x20, 0x20, 0x20, 0x20, 0x39, 0x39, 0x39, 0x39, 0x31, 0x31, 0x31, 0x31, 0x8F, 0xBC,
                0x58, 0x60,
            ]),
            otp_new(99991111)
        );

        assert_eq!(Err(()), otp_new(1234_5678_9012_3456));
    }
}
