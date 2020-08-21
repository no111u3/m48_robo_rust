#![no_std]
#![no_main]

extern crate panic_halt;

use atmega48p_hal::{atmega48p::Peripherals, clock, port::*, prelude::*, usart};

type Serial<IMODE> = usart::Usart0<clock::MHz1, IMODE>;

#[avr_device::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let mut serial = Serial::new(
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
