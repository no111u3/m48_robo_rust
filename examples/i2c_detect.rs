#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::{hal::i2c, prelude::*};

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let mut pinsc = dp.PORTC.split();

    let mut serial = m48_robo_rust::Serial::new(
        dp.USART0,
        pinsd.pd0,
        pinsd.pd1.into_output(&mut pinsd.ddr),
        2400,
    );

    let mut i2c = m48_robo_rust::I2c::new(
        dp.TWI,
        pinsc.pc4.into_pull_up_input(&mut pinsc.ddr),
        pinsc.pc5.into_pull_up_input(&mut pinsc.ddr),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "I2C detect from ATmega48P!\r").void_unwrap();

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, i2c::Direction::Write)
        .void_unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").void_unwrap();
    i2c.i2cdetect(&mut serial, i2c::Direction::Read)
        .void_unwrap();

    loop {}
}
