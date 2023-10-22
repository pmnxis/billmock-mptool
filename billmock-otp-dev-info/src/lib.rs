/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

#![no_std]

#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct OtpDeviceInfo {
    pub dev_sn: [u8; 12],
    pub crc: [u8; 4],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum OtpDeviceInfoParseErorr {
    /// All bytes are filled with 0xFF
    NotCarved,

    /// Serial number should be ASCII digits with left margin white-space
    NonAsciiDigit,

    /// Bad Checksum
    BadChecksum,
}

#[cfg(not(no_std))]
mod std_crc {
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

    pub fn is_ff_filled(&self) -> bool {
        let raw = unsafe {
            core::slice::from_raw_parts(
                (self as *const _) as *const u8,
                core::mem::size_of::<Self>(),
            )
        };

        for c in raw {
            if *c != 0xFF {
                return false;
            }
        }
        true
    }

    #[cfg(not(no_std))]
    pub fn new(sn: u64) -> Self {
        let mut ret = OtpDeviceInfo {
            dev_sn: [b' '; 12],
            crc: [0; 4],
        };
        let mut temp = sn.clone();

        for i in 0..ret.dev_sn.len() {
            let rev_i = ret.dev_sn.len() - i - 1;
            ret.dev_sn[rev_i] = (temp % 10) as u8 + b'0';
            temp /= 10;
            if temp == 0 {
                break;
            }
        }

        let checksum = std_crc::billmock_crc(&ret.dev_sn);
        ret.crc.copy_from_slice(&checksum.to_le_bytes());

        ret
    }

    #[cfg(not(no_std))]
    pub fn to_u64_arr(&self) -> [u64; 2] {
        let raw_data = unsafe {
            core::slice::from_raw_parts(
                (self as *const _) as *const u8,
                core::mem::size_of::<Self>(),
            )
        };

        let (first, second): ([u8; 8], [u8; 8]) = {
            let (mut r0, mut r1) = ([0u8; 8], [0u8; 8]);
            r0.copy_from_slice(&raw_data[0..8]);
            r1.copy_from_slice(&raw_data[8..16]);
            (r0, r1)
        };

        [u64::from_ne_bytes(first), u64::from_ne_bytes(second)]
    }

    #[cfg(not(no_std))]
    pub fn from_u64_arr(value: &[u64; 2]) -> Self {
        let ret = value.clone();

        unsafe { core::mem::transmute(ret) }
    }

    pub fn check_and_sn(&self) -> Result<(), OtpDeviceInfoParseErorr> {
        if !self.is_ascii_digit() {
            if self.is_ff_filled() {
                return Err(OtpDeviceInfoParseErorr::NotCarved);
            } else {
                return Err(OtpDeviceInfoParseErorr::NonAsciiDigit);
            }
        }

        #[cfg(not(no_std))]
        if std_crc::billmock_crc(&self.dev_sn).to_le_bytes() != self.crc {
            return Err(OtpDeviceInfoParseErorr::BadChecksum);
        }

        Ok(())
    }

    #[cfg(not(no_std))]
    pub fn check_and_sn_u64(&self) -> Result<u64, OtpDeviceInfoParseErorr> {
        self.check_and_sn()?;

        let mut ret = 0;
        let mut digit_mul = 1;

        for i in 0..self.dev_sn.len() {
            let rev_i = self.dev_sn.len() - i - 1;
            let ii = self.dev_sn[rev_i];

            if b'0' <= ii && ii <= b'9' {
                ret += digit_mul * (ii - b'0') as u64;
                digit_mul *= 10;
            } else {
                break;
            }
        }

        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

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

        use std_crc::billmock_crc;

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
        // 99991111
        let otp = OtpDeviceInfo::new(99991111);
        assert_eq!(
            [
                0x20, 0x20, 0x20, 0x20, 0x39, 0x39, 0x39, 0x39, 0x31, 0x31, 0x31, 0x31, 0x8F, 0xBC,
                0x58, 0x60,
            ],
            unsafe {
                core::slice::from_raw_parts(
                    (&otp as *const _) as *const u8,
                    core::mem::size_of_val(&otp),
                )
            }
        );
    }

    #[test]
    fn convert_u64_test() {
        assert_eq!(
            [0x3635343332312020, 0x127661D430393837],
            OtpDeviceInfo::new(1234567890).to_u64_arr()
        );

        assert_eq!(
            OtpDeviceInfo::from_u64_arr(&[0x3635343332312020, 0x127661D430393837]).to_u64_arr(),
            OtpDeviceInfo::new(1234567890).to_u64_arr()
        );
    }

    #[test]
    fn integrity_check() {
        assert!(matches!(
            OtpDeviceInfo::from_u64_arr(&[0x3635343332312020, 0x127661D430393837]).check_and_sn(),
            Ok(_)
        ));

        assert!(matches!(
            OtpDeviceInfo::from_u64_arr(&[0x0000000000000000, 0x127661D430393837]).check_and_sn(),
            Err(OtpDeviceInfoParseErorr::NonAsciiDigit)
        ));

        assert!(matches!(
            OtpDeviceInfo::from_u64_arr(&[0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]).check_and_sn(),
            Err(OtpDeviceInfoParseErorr::NotCarved)
        ));

        // under the `no_std` feature, no checksum check
        assert!(matches!(
            OtpDeviceInfo::from_u64_arr(&[0x3635343332312020, 0x0000000030393837]).check_and_sn(),
            Err(OtpDeviceInfoParseErorr::BadChecksum)
        ));

        assert!(matches!(
            OtpDeviceInfo::from_u64_arr(&[0x3635343332312020, 0x127661D430393837])
                .check_and_sn_u64(),
            Ok(1234567890)
        ));
    }
}
