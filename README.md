<!--
SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# `billmock-mptool`
![screenshot](./docs/example.png)

MPTool (Mass Production Tool) for [billmock-app-rs](https://github.com/pmnxis/billmock-app-rs)

## Billmock
Detail documentation is here [BillMock Manual](https://billmock.pmnxis.net/)

## Example
Toml
```
[database]
url = "postgres://billmockmptool:8sMkHadVg6whC9cYdkUC93op@10.59.0.10:7171/postgres"

[firmware]
path = "nda/billmock-app-rs-0v5-mini-7b4c4f7.elf"

[serial_number]
start = 10000001
end = 10099999
```

model_name, model_ver, firmware_ver, firmware_git_hash fields are automatically determined by elf header `.mp_fingerprint` that defined in `mp-fingerprint-type/src/lib.rs`.

Command Line
```sh
RUST_BACKTRACE=1 cargo run -- --config ./nda/20231029-BillMock-0v4-Mini.toml
```

### Hardware Setup
![hardware setup](./docs/IMG_6253.jpg)

1. Configure toml and firmware and just run
2. Connect stlink to board though pogo connector
3. Enter

## License
This program and the accompanying materials are made available under the terms
of the Apache Software License 2.0 which is available at
https://www.apache.org/licenses/LICENSE-2.0, or the MIT license which is 
available at https://opensource.org/licenses/MIT

Also all of codes are based one MIT or Apache Software License 2.0. But some common *.toml files are based on CC0-1.0 license. (Example Cargo.toml)
