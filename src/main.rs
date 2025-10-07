//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    loop {
        hprintln!("Hello, world!");
    }
}


// define the hard fault handler
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

// define the default exception handler
#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}