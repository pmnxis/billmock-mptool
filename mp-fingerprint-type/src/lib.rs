/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use std::path::Path;

use hex;
use serde::{Deserialize, Serialize};

use elf::endian::AnyEndian;
use elf::section::SectionHeader;
use elf::ElfBytes;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct FirmwareFingerprint {
    pub model_name: String,
    pub model_ver: String,
    pub firmware_ver: String,
    pub firmware_git_hash: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct MpFingerprint {
    pub firmware_fingerprint: FirmwareFingerprint,
}

impl MpFingerprint {
    /// convert fingerprint to upper hex string to use in build.rs
    pub fn to_hex_string(&self) -> String {
        hex::encode_upper(toml::to_string(self).expect("Serializing toml failed"))
    }

    pub fn from_elf<P: AsRef<Path>>(path: P) -> Self {
        let file_data = std::fs::read(path).expect("Could not read file.");
        let slice = file_data.as_slice();
        let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open failed");

        // Get the ELF file's build-id
        let mp_fingerprint_elf_section: SectionHeader = file
            .section_header_by_name(".mp_fingerprint")
            .expect("section table should be parseable")
            .expect("file should have a .note.ABI-tag section");

        let (mp_fingerprint_data, _) = file
            .section_data(&mp_fingerprint_elf_section)
            .expect("failed parse section data from elf");

        toml::from_str(std::str::from_utf8(mp_fingerprint_data).expect("incorrect data encoding"))
            .expect("Incorrect MpFingerprint toml format")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elf_test() {
        let path = std::path::PathBuf::from("testcase_elf_binary/otp-proof-of-concept");
        let actual = MpFingerprint::from_elf(path);

        let expected = MpFingerprint {
            firmware_fingerprint: FirmwareFingerprint {
                model_name: "otp-proof-of-concept".to_owned(),
                model_ver: "DUMMY-0V3".to_owned(),
                firmware_ver: "0.2.0".to_owned(),
                firmware_git_hash: "60b0d3d7075ffaab713ec0f85240829390328ec3".to_owned(),
            },
        };

        assert_eq!(actual, expected);
    }
}
