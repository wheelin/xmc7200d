#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m::asm;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    let periph = unsafe{xmc7200d_pac::Peripherals::steal()};

    loop {
        // your code goes here
    }
}
