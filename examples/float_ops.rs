#![no_std]
#![no_main]

extern crate panic_halt;

use atmega48p_hal::{atmega48p::Peripherals, clock, port::*, prelude::*, usart};
use micromath::F32Ext;

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

    let x = 4.5f32;

    let y = 2.9f32;

    let z = x + y * 1.5;

    ufmt::uwriteln!(
        &mut serial,
        "for x = 4.5 and y = 2.9 x + y * 1.5 = {}.{}\r",
        z.floor() as i32,
        (z.fract() * 100.0) as i32
    )
    .void_unwrap();

    loop {}
}
