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

use cortex_m::peripheral::{Peripherals, DWT};

use stm32f3xx_hal_v2::{self as hal, pac, prelude::*};

const VALID_ADDR_RANGE: Range<u8> = 0x40..0x55;
const MEMORY_ADDRESS: u16 = 0x0000; // Address to write/read data to/from
const MEMORY_ADDRESS_2: u16 = 0x0001; // Address to write/read data to/from

const CLOCK_FREQUENCY: u32 = 8_000_000; // Clock frequency in Hz


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

    let  core_peripherals = Peripherals::take().unwrap();
    let mut dwt = core_peripherals.DWT;
    dwt.enable_cycle_counter();

    // for addr in 0x48_u8..0x52 {
    //     // Write the empty array and check the slave response.
    //     if VALID_ADDR_RANGE.contains(&addr) && i2c.write(addr, &[]).is_ok() {
    //         hprint!("{:02x}", addr).unwrap();
    //     } else {
    //         hprint!("..").unwrap();
    //     }
    //     if addr % 0x10 == 0x0F {
    //         hprintln!().unwrap();
    //     } else {
    //         hprint!(" ").unwrap();
    //     }
    // }


    let value = 0x0C;
     let buff = [
        ((MEMORY_ADDRESS >> 8) & 0xFF) as u8,
        (MEMORY_ADDRESS & 0xFF) as u8,
        value
    ];
    // Write the buffer to the I2C device
    
   let start_write = DWT::cycle_count();
   i2c.write(fram_address, &buff).unwrap();
   let end_write = DWT::cycle_count();
   let write_time = ((end_write - start_write) as f32 / CLOCK_FREQUENCY as f32) * 1000.0;

   hprintln!("Write time: {} ms", write_time).unwrap();

    let value = 0x0A;
    let buff = [
       ((MEMORY_ADDRESS_2 >> 8) & 0xFF) as u8,
       (MEMORY_ADDRESS_2 & 0xFF) as u8,
       value
   ];

   let start_write = DWT::cycle_count();
   i2c.write(fram_address, &buff).unwrap();
   let end_write = DWT::cycle_count();
   let write_time = ((end_write - start_write) as f32 / CLOCK_FREQUENCY as f32) * 1000.0;

   hprintln!("Write time: {} ms", write_time).unwrap();


   let mut data = [0;1];
   let memory_address_bytes = [(MEMORY_ADDRESS >> 8) as u8, MEMORY_ADDRESS as u8];
   let start_read = DWT::cycle_count();
   i2c.write_read(fram_address, &memory_address_bytes, &mut data).unwrap();
   let end_read = DWT::cycle_count();
   let read_time = ((end_read - start_read) as f32 / CLOCK_FREQUENCY as f32) * 1000.0;
   hprintln!("Read time: {} ms", read_time).unwrap();
   hprintln!("Data read: {:?}", data).unwrap();

   let mut data = [0;1];
   let memory_address_bytes = [(MEMORY_ADDRESS_2 >> 8) as u8, MEMORY_ADDRESS_2 as u8];
   let start_read = DWT::cycle_count();
   i2c.write_read(fram_address, &memory_address_bytes, &mut data).unwrap();
   let end_read = DWT::cycle_count();
   let read_time = ((end_read - start_read) as f32 / CLOCK_FREQUENCY as f32) * 1000.0;

    hprintln!("Read time: {} ms", read_time).unwrap();
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