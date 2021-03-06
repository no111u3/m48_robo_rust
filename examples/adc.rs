#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::{adc, prelude::*};

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

    ufmt::uwriteln!(&mut serial, "ADC example ATmega48P!\r").void_unwrap();

    let mut adc = adc::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd): (u16, u16) = (
        nb::block!(adc.read(&mut adc::channel::Vbg)).void_unwrap(),
        nb::block!(adc.read(&mut adc::channel::Gnd)).void_unwrap(),
    );

    ufmt::uwriteln!(&mut serial, "Vbandgap: {}\r", vbg).void_unwrap();
    ufmt::uwriteln!(&mut serial, "GND: {}\r", gnd).void_unwrap();

    let portc = dp.PORTC.split();
    let mut a0 = portc.pc5.into_analog_input(&mut adc);
    let mut a1 = portc.pc4.into_analog_input(&mut adc);
    let mut a2 = portc.pc3.into_analog_input(&mut adc);
    let mut a3 = portc.pc2.into_analog_input(&mut adc);
    let mut a4 = portc.pc1.into_analog_input(&mut adc);
    let mut a5 = portc.pc0.into_analog_input(&mut adc);

    loop {
        let values: [u16; 6] = [
            nb::block!(adc.read(&mut a0)).void_unwrap(),
            nb::block!(adc.read(&mut a1)).void_unwrap(),
            nb::block!(adc.read(&mut a2)).void_unwrap(),
            nb::block!(adc.read(&mut a3)).void_unwrap(),
            nb::block!(adc.read(&mut a4)).void_unwrap(),
            nb::block!(adc.read(&mut a5)).void_unwrap(),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).void_unwrap();
        }
        ufmt::uwriteln!(&mut serial, "\r").void_unwrap();

        m48_robo_rust::delay_ms(100);
    }
}
