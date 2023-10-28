/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use std::path::Path;

pub struct Firmware {
    pub path: String,
    pub model_name: String,
    pub model_ver: String,
    pub firmware_ver: String,
    pub firmware_git_hash: String,
}
