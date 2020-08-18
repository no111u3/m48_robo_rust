#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use atmega48p_hal::{atmega48p::Peripherals, port::*, prelude::*};

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let mut pb0 = portb.pb0.into_output(&mut portb.ddr);

    loop {
        pb0.set_high().void_unwrap();

        small_delay();

        pb0.set_low().void_unwrap();

        small_delay();
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..10000 {
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}
