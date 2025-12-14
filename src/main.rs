//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use cortex_m;
use panic_semihosting as _;
use stm32wb::{self as _};

use cortex_m_rt::{ExceptionFrame, entry, exception};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let proc_periphs = cortex_m::Peripherals::take().unwrap();
    let mut systick = proc_periphs.SYST;
    systick.clear_current();
    systick.set_reload(1024);
    systick.enable_counter();
    systick.enable_interrupt();
    loop {
        // hprintln!("Hello, world!");
    }
}

#[exception]
unsafe fn SysTick() {
    hprintln!("Timer looped");
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
