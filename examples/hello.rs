//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};


#[entry]
fn main() -> ! {

    if let Some(dp) = Peripherals::take() {
        // RCC (Reset and Clock Control) peripheral
        let rcc = dp.RCC.constrain();

        // Clocks
        let clocks = rcc.cfgr.freeze();

        // Get the system clock frequency
        let sysclk = clocks.sysclk().0;

        // Print the system clock frequency
        defmt::info!("System Clock Frequency: {} Hz", sysclk);

    }
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
   debug::exit(debug::EXIT_SUCCESS);

    loop {}
}