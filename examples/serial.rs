#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::prelude::*;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let mut serial = m48_robo_rust::Serial::new(
        dp.USART0,
        pinsd.pd0,
        pinsd.pd1.into_output(&mut pinsd.ddr),
        2400,
    );

    ufmt::uwriteln!(&mut serial, "Echo from ATmega48P!\r").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        ufmt::uwrite!(&mut serial, "{}", b as char).void_unwrap();
    }
}
