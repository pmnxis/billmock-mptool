/*
 * SPDX-FileCopyrightText: © 2023 Jinwoo Park (pmnxis@gmail.com)
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

#![no_main]
#![no_std]
#![feature(const_trait_impl)]
#![feature(async_fn_in_trait)]
#![feature(type_alias_impl_trait)]
#![feature(effects)] // see : https://github.com/rust-lang/rust/issues/114808

use embassy_executor::Spawner;
use embassy_stm32::crc::{Config as CrcConfig, Crc, InputReverseConfig};
use embassy_stm32::exti::{Channel as HwChannel, ExtiInput};
use embassy_stm32::flash::Flash;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_time::{Duration, Timer};
use static_cell::make_static;
use zeroable::Zeroable;
use {defmt_rtt as _, panic_probe as _};
use {defmt_rtt as _, panic_probe as _};
bind_interrupts!(struct Irqs {
    USART2 => embassy_stm32::usart::InterruptHandler<peripherals::USART2>;
    I2C1 => embassy_stm32::i2c::InterruptHandler<peripherals::I2C1>;
});

#[repr(C)]
#[derive(Zeroable, Clone, Copy)]
pub struct DeviceSerialNumberEeprom {
    pub dev_sn: [u8; 12],
    pub crc: [u8; 4],
}

impl DeviceSerialNumberEeprom {
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
}

#[repr(C)]
#[derive(Zeroable)]
pub struct AbsoluteRamSpace {
    pub margin_top: [u8; 8],
    pub data: DeviceSerialNumberEeprom,
    pub margin_bot: [u8; 8],
}

pub type EepromAddress = u8;
pub type DevSelAddress = u8;
pub const DEVICE_SERIAL_NUMBER_EEPROM_ADDRESS: u16 = 2032;
const ROM_7B_ADDRESS: u8 = 0b1010000; // Embassy require 7bits address as parameter.
                                      // const ROM_ADDRESS_FIELD_SIZE: usize = core::mem::size_of::<u8>();
const WAIT_DURATION_PER_PAGE: Duration = Duration::from_millis(40); // heuristic value
pub const FLASH_DEVICE_INFO_OFFSET: u32 = 0xF800;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    // hardware 0.4 and 0.4-mini have same I2C pin-out
    // #[cfg(all(feature = "hw_mini_0v4", not(feature = "hw_0v2"), not(feature = "hw_0v3"), not(feature = "hw_0v4")))]
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH4,
        p.DMA1_CH3,
        Hertz(400_000),
        Default::default(),
    );

    let mut nwp =
        embassy_stm32::gpio::OutputOpenDrain::new(p.PF0, Level::High, Speed::Low, Pull::None);

    let mut led0 = Output::new(p.PA4, Level::Low, Speed::Low);
    let mut led1 = Output::new(p.PA5, Level::Low, Speed::Low);

    // InputReverseConfig::Halfword
    let Ok(crc_config) = embassy_stm32::crc::Config::new(InputReverseConfig::Word, false, 0xA097)
    else {
        panic!("Something went horribly wrong")
    };
    let mut crc = Crc::new(p.CRC, crc_config);

    // let flash_space = unsafe {
    //     core::slice::from_raw_parts(
    //         0x0800_F800 as *const _,
    //         core::mem::size_of::<DeviceSerialNumberEeprom>(),
    //     )
    // };

    let flash_space: &DeviceSerialNumberEeprom = unsafe { core::mem::transmute(0x0800_F800) };
    let flash_space_as_arr: &[u8; 16] = unsafe { core::mem::transmute(0x0800_F800) };
    let is_flash_available = {
        if flash_space.is_ascii_digit() == false {
            false
        } else {
            crc.reset();
            let checksum = crc.feed_bytes(&flash_space.dev_sn);
            let flash_checksum_expected = checksum.to_be_bytes();
            let flash_checksum_actual = flash_space.crc;
            defmt::info!(
                "flash crc expected 0x{:02X}, actual 0x{:02X}\n Checksum : {:08X}",
                flash_checksum_expected,
                flash_checksum_actual,
                checksum,
            );
            defmt::info!("flash_space {:?}", flash_space.crc);

            flash_checksum_actual == flash_checksum_expected
        }
    };

    defmt::info!("flash space force read : {:#X}", flash_space_as_arr);

    let data_address_slice = (DEVICE_SERIAL_NUMBER_EEPROM_ADDRESS as EepromAddress).to_be_bytes();
    let i2c_address =
        ROM_7B_ADDRESS | ((DEVICE_SERIAL_NUMBER_EEPROM_ADDRESS >> 8) as DevSelAddress & 0x7);

    let mut rx_buffer: [u8; core::mem::size_of::<DeviceSerialNumberEeprom>()] =
        [0; core::mem::size_of::<DeviceSerialNumberEeprom>()];
    let rx_buffer_cast: &mut DeviceSerialNumberEeprom =
        unsafe { core::mem::transmute(rx_buffer.as_ptr()) };

    let mut raw_tx_buffer = [0u8; core::mem::size_of::<DeviceSerialNumberEeprom>() + 4];
    let tx_buffer_cast: &mut DeviceSerialNumberEeprom =
        unsafe { core::mem::transmute(raw_tx_buffer[4..].as_ptr()) };
    // let mut i2c_buffer: [u8; core::mem::size_of::<DeviceSerialNumberEeprom>() + 2] =
    //     [0; core::mem::size_of::<DeviceSerialNumberEeprom>() + 2];
    // let rx_buffer = &mut i2c_buffer[2..];

    let is_eeprom_available = match i2c.blocking_write_read_timeout(
        i2c_address,
        &data_address_slice,
        &mut rx_buffer,
        WAIT_DURATION_PER_PAGE,
    ) {
        Ok(_) => {
            // Determine EEPROM status
            crc.reset();

            let eeprom_checksum_expected = crc.feed_bytes(&rx_buffer_cast.dev_sn).to_be_bytes();
            let eeprom_checksum_actual = rx_buffer_cast.crc;

            defmt::debug!(
                "eeprom : {:#X}\n{:#X} {:#X}",
                rx_buffer,
                eeprom_checksum_expected,
                eeprom_checksum_actual
            );

            defmt::debug!("eeprom.dev_sn : {:#X}", rx_buffer_cast.dev_sn);

            eeprom_checksum_expected == eeprom_checksum_actual
        }
        Err(e) => {
            defmt::error!("blocking_write_read_timeout (initial), {:?}", e);

            false
        }
    };

    defmt::info!(
        "(is_flash_available, is_eeprom_available) : {}, {}",
        is_flash_available,
        is_eeprom_available
    );

    match (is_flash_available, is_eeprom_available) {
        (true, false) => {
            // FLASH -> EEPROM
            // Maybe initial mass-production
            defmt::info!("Start FLASH -> EEPROM");

            *tx_buffer_cast = flash_space.clone();

            let tx_addr_buffer = &mut raw_tx_buffer[(4 - core::mem::size_of::<EepromAddress>())..4];
            tx_addr_buffer.copy_from_slice(&data_address_slice);
            let final_tx_buffer = &raw_tx_buffer[(4 - core::mem::size_of::<EepromAddress>())..];

            nwp.set_low(); // EEPROM write unlock

            if let Err(e) =
                i2c.blocking_write_timeout(i2c_address, final_tx_buffer, WAIT_DURATION_PER_PAGE)
            {
                defmt::error!("blocking_write_timeout, {:?}", e);
            }

            nwp.set_high(); // EEPROM write lock
            Timer::after(Duration::from_millis(5)).await;

            if let Err(e) = i2c.blocking_write_read_timeout(
                i2c_address,
                &data_address_slice,
                &mut rx_buffer,
                WAIT_DURATION_PER_PAGE,
            ) {
                defmt::error!("blocking_write_read_timeout, {:?}", e);
            }

            defmt::info!("EEPROM double check : {:#X}", rx_buffer);
        }
        (false, true) => {
            // EEPROM -> RAM and FLASH
            defmt::info!("Start EEPROM -> FLASH");

            if let Err(e) =
                f.blocking_erase(FLASH_DEVICE_INFO_OFFSET, FLASH_DEVICE_INFO_OFFSET + 1024)
            {
                defmt::error!("Failed to erase last page on FLASH {:?}", e);
            }
\
            if let Err(e) = f.blocking_write(FLASH_DEVICE_INFO_OFFSET, &rx_buffer) {
                defmt::error!("Failed to write last page on FLASH {:?}", e);
            }

            defmt::info!("EEPROM -> FLASH: {:#X}", flash_space_as_arr);
        }
        (false, false) => {
            // Critical Report Issue and write dummy
        }
        (true, true) => {
            // Server Need decide thing
        }
    }
}
