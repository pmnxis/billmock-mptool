/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

mod config;
// mod mp_flash;
// mod firmware;
mod programmer;

use billmock_otp_dev_info::*;
use mp_fingerprint_type::MpFingerprint;
// use clap::Parser;
#[allow(unused)]
use probe_rs::{
    flashing::{self, DownloadOptions, FlashLoader},
    Error, MemoryInterface, Permissions, Session,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder,
    Set, TransactionTrait,
};

use clap::Parser;

#[derive(clap::Parser)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, short = 'c', value_name = "CONFIG")]
    config: String,

    /// Unlocked state
    #[clap(long, short = 'u', action = clap::ArgAction::SetTrue)]
    unlocked: bool,

    /// Grab/Give serial number process only, do not flash
    #[clap(long, short = 'g', action = clap::ArgAction::SetTrue)]
    sn_only: bool,
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = config::get_config(&ARGS.config);

    println!("config : {:?}", config);

    println!("try to connect database");

    let db_conn = match sea_orm::Database::connect(config.database.url.clone()).await {
        Ok(x) => Ok(x),
        Err(e) => {
            println!("Database error : {:?}", e);
            Err(e)
        }
    }?;

    println!("database connected");

    let mut batch_count = 0;

    loop {
        let firmware_path = std::path::PathBuf::from(&config.firmware.path);
        let firmware_ext = firmware_path
            .extension()
            .map(|x| x.to_ascii_lowercase().to_string_lossy().to_string());

        let fingerprint = MpFingerprint::from_elf(&firmware_path).firmware_fingerprint;

        if batch_count == 0 {
            println!("fingerprint : {:?}", fingerprint);
        }

        if !fingerprint.is_nda {
            println!("Firmware binary is not NDA build. If you agree to flash, type `agree`, else type other keyword");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("stdin error");

            if input.trim().to_lowercase() != "agree" {
                println!(
                    "Exit the job because disagree non-NDA flashing {}",
                    batch_count
                );
                break;
            }
        }

        println!("If you ready press any key, or if you wanna stop batch task press `Q`.");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("stdin error");

        if input.trim() == "q" {
            println!("Exit the job, total batch task : {}", batch_count);
            // stop batch job
            break;
        }

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

        // flash firmware

        if rdp.is_ok() && !ARGS.sn_only {
            let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;
            // let mut loader = session.target().flash_loader();

            if !firmware_path.exists() {
                panic!("firmware is not found");
            }

            if firmware_ext == Some(String::from("hex")) {
                flashing::download_file(&mut session, firmware_path, flashing::Format::Hex)?;
            } else if firmware_ext == Some(String::from("elf")) || firmware_ext.is_none() {
                flashing::download_file(&mut session, firmware_path, flashing::Format::Elf)?;
            } else {
                panic!("Unsupported extension");
            }

            println!("Firmware flashed");
        }

        if rdp.is_ok() {
            let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;
            let current = chrono::Utc::now();

            if let Ok(raw_otp) = programmer::get_otp(&mut session) {
                let otp = OtpDeviceInfo::from_u64_arr(&raw_otp);
                match otp.check_and_sn_u64() {
                    Ok(sn) => {
                        // report server, this is second time
                        println!("Already have serial number : {:12}", sn);

                        // update to server
                        let sn_i64: i64 = sn.try_into().unwrap();

                        db_conn
                            .transaction::<_, _, DbErr>(|txn| {
                                Box::pin(async move {
                                    let model = entity::hardware::Entity::find_by_id(sn_i64)
                                        .one(txn)
                                        .await?;

                                    match model {
                                        None => {
                                            let new_sn_am = entity::hardware::ActiveModel {
                                                id: Set(sn_i64),
                                                model_name: Set(fingerprint.model_name.clone()),
                                                model_ver: Set(fingerprint.model_ver.clone()),

                                                initial_firmware_ver: Set(fingerprint
                                                    .firmware_ver
                                                    .clone()),
                                                initial_firmware_git_hash: Set(fingerprint
                                                    .firmware_git_hash
                                                    .clone()),
                                                // latest firmware field
                                                latest_firwmare_ver: Set(fingerprint
                                                    .firmware_ver
                                                    .clone()),
                                                latest_firwmare_git_hash: Set(fingerprint
                                                    .firmware_git_hash
                                                    .clone()),

                                                register_time: Set(current.into()), // initial firmware field
                                                latest_update_time: Set(current.into()), // latest firmware field
                                            };

                                            new_sn_am.insert(txn).await?;
                                            println!("Alternative insert new hardware on db.")
                                        }
                                        Some(x) => {
                                            let mut am = x.into_active_model();
                                            am.latest_firwmare_ver =
                                                Set(fingerprint.firmware_ver.clone());
                                            am.latest_firwmare_git_hash =
                                                Set(fingerprint.firmware_git_hash.clone());
                                            am.latest_update_time = Set(current.into());

                                            am.update(txn).await?;
                                            println!(
                                                "Update latest field for existing hardware on db."
                                            )
                                        }
                                    }

                                    Ok(())
                                })
                            })
                            .await?;
                        // end of update to server
                    }
                    Err(OtpDeviceInfoParseErorr::NotCarved) => {
                        // get new serial number from server
                        let sn_vec = entity::hardware::Entity::find()
                            .filter(
                                entity::hardware::Column::Id
                                    .between(config.serial_number.start, config.serial_number.end),
                            )
                            .order_by_asc(entity::hardware::Column::Id)
                            .all(&db_conn)
                            .await?;

                        let mut new_sn = config.serial_number.start;

                        for sn in sn_vec {
                            let cmp_new_sn: i64 = new_sn.try_into().unwrap();
                            let existing = sn.id;

                            if existing != cmp_new_sn {
                                break;
                            } else {
                                new_sn = (cmp_new_sn + 1) as u64;
                            }
                        }
                        // end of get new serial number from server

                        // programming otp
                        let otp_u64_arr = OtpDeviceInfo::new(new_sn).to_u64_arr();
                        programmer::set_otp(&mut session, otp_u64_arr)?;

                        println!("New board detected, write to : {:12}", new_sn);
                        // end of programming otp

                        // update to server
                        let new_sn_am = entity::hardware::ActiveModel {
                            id: Set(new_sn.try_into().unwrap()),
                            model_name: Set(fingerprint.model_name.clone()),
                            model_ver: Set(fingerprint.model_ver.clone()),

                            initial_firmware_ver: Set(fingerprint.firmware_ver.clone()),
                            initial_firmware_git_hash: Set(fingerprint.firmware_git_hash.clone()),
                            // latest firmware field
                            latest_firwmare_ver: Set(fingerprint.firmware_ver.clone()),
                            latest_firwmare_git_hash: Set(fingerprint.firmware_git_hash.clone()),

                            register_time: Set(current.into()), // initial firmware field
                            latest_update_time: Set(current.into()), // latest firmware field
                        };

                        new_sn_am.insert(&db_conn).await?;
                        // end of update to server
                    }
                    Err(e) => {
                        println!("Serial number has problem {:?}", e);
                    }
                }
            }
        }

        // lock firmware again
        if !ARGS.unlocked {
            let mut session = Session::auto_attach("STM32G030C8Tx", Permissions::default())?;

            let prev = programmer::get_rdp(&mut session)?;

            if prev == 0xBB {
                println!("Skip writing RDP");
            } else {
                programmer::set_rdp(&mut session, 0xBB)?;
                println!("prev : {:2X} ---> next : {:2X}", prev, 0xBB);
            }

            let _ = session.core(0)?.reset();

            drop(session);
        }
        // end of lock firmware again

        batch_count += 1;
    }

    Ok(())
}
