#![no_main]
#![no_std]

use cortex_app as _; // global logger + panicking-behavior + memory layout
use cortex_m_semihosting::{hprintln};

#[cortex_m_rt::entry]
fn main() -> ! {
    // defmt::println!("Hello, world!");
    hprintln!("Hello, world!");
    cortex_app::exit()
}
