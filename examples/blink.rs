#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::prelude::*;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let mut pb0 = portb.pb0.into_output(&mut portb.ddr);

    loop {
        pb0.set_high().void_unwrap();

        m48_robo_rust::delay_ms(200);

        pb0.set_low().void_unwrap();

        m48_robo_rust::delay_ms(200);
    }
}
