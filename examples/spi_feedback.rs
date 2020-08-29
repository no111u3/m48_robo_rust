#![no_std]
#![no_main]

extern crate panic_halt;
use m48_robo_rust::spi::{Settings, Spi};
use m48_robo_rust::{delay_ms, prelude::*};
use nb::block;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let mut pinsb = dp.PORTB.split();

    let mut serial = m48_robo_rust::Serial::new(
        dp.USART0,
        pinsd.pd0,
        pinsd.pd1.into_output(&mut pinsd.ddr),
        2400,
    );

    pinsb.pb2.into_output(&mut pinsb.ddr); // SS must be set to output mode.

    let mut spi = Spi::new(
        dp.SPI,
        pinsb.pb5.into_output(&mut pinsb.ddr),
        pinsb.pb3.into_output(&mut pinsb.ddr),
        pinsb.pb4.into_pull_up_input(&mut pinsb.ddr),
        Settings::default(),
    );

    loop {
        // Send a byte
        block!(spi.send(0b00001111)).void_unwrap();
        // Because MISO is connected to MOSI, the read data should be the same
        let data = block!(spi.read()).void_unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        delay_ms(1000u16);
    }
}
