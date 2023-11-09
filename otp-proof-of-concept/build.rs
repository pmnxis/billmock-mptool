/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use cargo_metadata::MetadataCommand;
use git2::Repository;
use mp_fingerprint_type::{FirmwareFingerprint, MpFingerprint};

fn main() -> Result<(), ()> {
    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");

    // Generate elf header fingerprint
    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let main_package = metadata
        .packages
        .first()
        .expect("Cargo.toml doesn't have metadata");

    // Get the Git commit hash
    // this sub-project's git root is "..", normal case "."
    let repo = Repository::open("..").expect("Failed to open repository");
    let head = repo.head().expect("Failed to get HEAD");
    let commit = head.peel_to_commit().expect("Failed to peel commit");
    let commit_hash = commit.id().to_string();

    let hw_feature: Vec<(String, String)> = std::env::vars()
        .filter(|(key, value)| key.starts_with("CARGO_FEATURE_HW_") && value == "1")
        .collect();

    if hw_feature.is_empty() {
        panic!("There's no specified hardware target");
    } else if hw_feature.len() > 1 {
        panic!("Cannot specify multiple hardware");
    }

    let feature_based_model_ver = hw_feature[0]
        .0
        .strip_prefix("CARGO_FEATURE_HW_")
        .unwrap()
        .replace("_", "-");

    let fingerprint = MpFingerprint {
        firmware_fingerprint: FirmwareFingerprint {
            model_name: main_package.name.clone(), // reference package name temporary
            model_ver: feature_based_model_ver,
            firmware_ver: main_package.version.to_string(),
            firmware_git_hash: format!("{}", commit_hash),
            is_nda: false,
        },
    };

    // cargo objdump --release -- -s --section .mp_fingerprint
    println!(
        "cargo:rustc-env=MP_FINGERPRINT_TOML_HEX={}",
        fingerprint.to_hex_string(),
    );

    Ok(())
}
