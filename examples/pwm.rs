#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use atmega48p_hal::{atmega48p::Peripherals, port::*, prelude::*, pwm::*};

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let mut timer = Timer1Pwm::new(dp.TC1, Prescaler::Prescale8);

    let mut pin = portb.pb1.into_output(&mut portb.ddr).into_pwm(&mut timer);

    pin.set_duty(128);
    pin.enable();

    loop {
        for i in 0..=255u16 {
            let duty: u16 = i * i / 256;
            pin.set_duty(duty as u8);
            small_delay();
        }
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..100 {
        unsafe { llvm_asm!("" :::: "volatile") }
    }
}
