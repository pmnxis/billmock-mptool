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
url = "postgres://postgres:postgres@localhost:5432/postgres"

[firmware]
path = "nda/billmock-app-rs-0v4-mini-0ca2b4e.elf"
model_name = "BillMock-HW"
model_ver = "0.4-mini"
firmware_ver = "0.2.1"
firmware_git_hash = "0ca2b4e6f534c61e9786fe27bc87d95c05413e74"

[serial_number]
start = 10000001
end = 19999999

```

Command Line
```sh
RUST_BACKTRACE=1 cargo run -- --config ./nda/20231029-BillMock-0v4-Mini.toml
```

## License
This program and the accompanying materials are made available under the terms
of the Apache Software License 2.0 which is available at
https://www.apache.org/licenses/LICENSE-2.0, or the MIT license which is 
available at https://opensource.org/licenses/MIT

Also all of codes are based one MIT or Apache Software License 2.0. But some common *.toml files are based on CC0-1.0 license. (Example Cargo.toml)
