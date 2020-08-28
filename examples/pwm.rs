#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::prelude::*;
use m48_robo_rust::pwm;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let mut timer = pwm::Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale8);

    let mut pin = portb.pb1.into_output(&mut portb.ddr).into_pwm(&mut timer);

    pin.set_duty(128);
    pin.enable();

    loop {
        for i in 0..=255u16 {
            let duty: u16 = i * i / 256;
            pin.set_duty(duty as u8);
            m48_robo_rust::delay_ms(10);
        }
    }
}
