/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct Firmware {
    pub path: String,
}

#[derive(Clone, Deserialize)]
pub struct SerialNumber {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub database: Database,
    pub firmware: Firmware,
}
