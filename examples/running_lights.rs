#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::{delay_ms, hal::port::*, prelude::*};

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let pb6 = portb.pb6.into_output(&mut portb.ddr);
    let pb7 = portb.pb7.into_output(&mut portb.ddr);
    let pb0 = portb.pb0.into_output(&mut portb.ddr);
    let pb1 = portb.pb1.into_output(&mut portb.ddr);

    let mut leds: [Pin<mode::Output>; 4] = [
        pb6.downgrade(),
        pb7.downgrade(),
        pb0.downgrade(),
        pb1.downgrade(),
    ];

    loop {
        for i in 0..leds.len() {
            leds[i].set_high().void_unwrap();

            leds[if i < 1 { leds.len() - 1 } else { i - 1 }]
                .set_low()
                .void_unwrap();

            delay_ms(200);
        }
    }
}
