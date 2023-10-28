/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Hardware::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Hardware::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Hardware::ModelName).string().not_null())
                    .col(ColumnDef::new(Hardware::ModelVer).string().not_null())
                    .col(
                        ColumnDef::new(Hardware::InitialFirmwareVer)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hardware::InitialFirmwareGitHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hardware::LatestFirwmareVer)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hardware::LatestFirwmareGitHash)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hardware::RegisterTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Hardware::LatestUpdateTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Hardware::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Hardware {
    Table,
    Id,
    ModelName,
    ModelVer,
    InitialFirmwareVer,
    InitialFirmwareGitHash,
    LatestFirwmareVer,
    LatestFirwmareGitHash,
    RegisterTime,
    LatestUpdateTime,
}
