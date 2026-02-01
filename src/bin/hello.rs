#![no_main]
#![no_std]

use cortex_app as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    cortex_app::exit()
}
