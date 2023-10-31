/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct FirmwareConfig {
    pub path: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SerialNumberBoundary {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub firmware: FirmwareConfig,
    pub serial_number: SerialNumberBoundary,
}

pub fn get_config(path: &str) -> Config {
    match std::fs::read_to_string(path) {
        Ok(x) => toml::from_str(&x).expect("Please select corresponding toml file"),
        Err(_) => panic!("Cannot read toml file"),
    }
}
