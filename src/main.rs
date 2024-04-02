//! Example of using I2C.
//! Scans available I2C devices on bus and print the result.
//! Appropriate pull-up registers should be installed on I2C bus.
//! Target board: STM32F3DISCOVERY

#![no_std]
#![no_main]

use panic_halt as _;
use core::ops::Range;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};

use stm32f3xx_hal_v2::{self as hal, pac, prelude::*, i2c::Error};

const VALID_ADDR_RANGE: Range<u8> = 0x40..0x55;
const MEMORY_ADDRESS: u16 = 0x0000; // Address to write/read data to/from
const DEVICE_ADDRESS: u8 = 0x50; // I2C device address


fn check_i2c_error(error: Error) -> Result<(), ()> {
    match error {
        Error::Arbitration => {
            hprintln!("Arbitration loss occurred").unwrap();
            Err(())
        }
        Error::Bus => {
            hprintln!("Bus error occurred").unwrap();
            Err(())
        }
        Error::Busy => {
            hprintln!("Bus is busy").unwrap();
            Err(())
        }
        Error::Nack => {
            hprintln!("NACK received during transfer").unwrap();
            Err(())
        }
        _ => {
            hprintln!("Unknown error occurred").unwrap();
            Err(())
        }

    }
}


#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    let  scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh); // SCL
    let  sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh); // SDA

    // Configure I2C1
    let pins = (scl, sda);

    
    let mut i2c = hal::i2c::I2c::new(dp.I2C1, pins, 100.khz(), clocks, &mut rcc.apb1);

    let fram_address = 0x50;


    for addr in 0x48_u8..0x52 {
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
     // Read data from the memory location
     let memory_address_bytes = [(MEMORY_ADDRESS >> 8) as u8, MEMORY_ADDRESS as u8];
     let value = 0xAC;
     let buff = [
        ((MEMORY_ADDRESS >> 8) & 0xFF) as u8,
        (MEMORY_ADDRESS & 0xFF) as u8,
        value
    ];

    // Write the buffer to the I2C device
    i2c.write(fram_address, &buff).unwrap();

    let mut data = [0;100];
    i2c.write_read(fram_address, &memory_address_bytes, &mut data).unwrap();
    hprintln!("Data read: {:?}", data).unwrap();
    //{
    //     Ok(()) => {
    //         //let data =  data[0];
    //         hprintln!("Data read: {:?}", data).unwrap();
    //     }
    //     Err(e) => {
    //         check_i2c_error(e).unwrap();
    //         loop {} // Exit if error occurs
    //     }
    // }
    loop {
       
    }
}