//! Example of using I2C.
//! Scans available I2C devices on bus and print the result.
//! Appropriate pull-up registers should be installed on I2C bus.
//! Target board: STM32F3DISCOVERY

#![no_std]
#![no_main]

use core::ops::Range;

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

use stm32f3xx_hal_v2::{self as hal, pac, prelude::*};

const FRAM_ADDR: u8 = 0x50; // I2C device address
const MEMORY_ADDRESS: u16 = 0x0001; // Address to write/read data to/from

const VALID_ADDR_RANGE: Range<u8> = 0x08..0x78;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Configure I2C1
    let pins = (
        gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl), // SCL
        gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl), // SDA
    );
    let mut i2c = hal::i2c::I2c::new(dp.I2C1, pins, 100.khz(), clocks, &mut rcc.apb1);

    hprintln!("Start i2c scanning...").expect("Error using hprintln.");
   // hprintln!().unwrap();


      // Data to write to F-RAM
      let data_to_write: u8 = 0xAB;

      // Write data to the specified memory location
      let address_bytes: [u8; 2] = [(MEMORY_ADDRESS >> 8) as u8, MEMORY_ADDRESS as u8];
    
      i2c.write(FRAM_ADDR, &address_bytes).unwrap();
      
      match i2c.write(FRAM_ADDR, &[data_to_write]) {
        Ok(_) => {
            hprintln!("Success!!").unwrap();
        }
        Err(_) => {
            hprintln!("Error!!").unwrap();
        }
    }

        // Read data from the specified memory location
     let mut data_read: [u8; 1] = [0];
    match i2c.write_read(FRAM_ADDR, &address_bytes, &mut data_read){
        Ok(_) => {
            // Data read successfully
            let data = data_read[0];
            hprintln!("{}", data).unwrap();
            // You can print or process the data as needed
        }
        Err(_) => {
            hprintln!("Error reading data!!").unwrap();
            // Error reading data
        }
    }

    //hprintln!("{}", data_read[0]).unwrap();
    hprintln!("Done!").unwrap();


     // Write 8-bit value to the fixed address
    // let data_to_write: [u8; 2] = [MEMORY_ADDRESS, 0xAB]; // Combine memory address and data


    for addr in 0x00_u8..0x80 {
        // Write the empty array and check the slave response.
        if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok() {
            hprint!("{:02x}", addr).unwrap();
        } else {
            hprint!("..").unwrap();
        }
        if addr % 0x10 == 0x0F {
            hprintln!().unwrap();
        } else {
            hprint!(" ").unwrap();
        }
    }

    // hprintln!().unwrap();
    // hprintln!("Done!").unwrap();

    loop {
    
    }
}