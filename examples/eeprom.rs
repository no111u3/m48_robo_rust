#![no_std]
#![no_main]

extern crate panic_halt;
use m48_robo_rust::prelude::*;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let eeprom = dp.EEPROM;

    let mut serial = m48_robo_rust::Serial::new(
        dp.USART0,
        pinsd.pd0,
        pinsd.pd1.into_output(&mut pinsd.ddr),
        2400,
    );

    ufmt::uwriteln!(&mut serial, "MCU EEPROM write/read ATmega48P!\r").void_unwrap();

    let mut counter = ee_read(&eeprom, 0);

    ufmt::uwriteln!(&mut serial, "Counter readed from EEPROM: {}\r", counter).void_unwrap();

    counter = counter.overflowing_add(1).0;

    ufmt::uwriteln!(&mut serial, "Counter writed to EEPROM: {}\r", counter).void_unwrap();

    ee_write(&eeprom, 0, counter);

    loop {}
}

fn ee_write(eeprom: &m48_robo_rust::atmega48p::EEPROM, address: u8, value: u8) {
    while eeprom.eecr.read().eepe().bit_is_set() {}

    eeprom.eearl.write(|w| unsafe { w.bits(address) });
    eeprom.eedr.write(|w| unsafe { w.bits(value) });

    eeprom.eecr.modify(|_, w| w.eempe().set_bit());
    eeprom.eecr.modify(|_, w| w.eepe().set_bit());
}

fn ee_read(eeprom: &m48_robo_rust::atmega48p::EEPROM, address: u8) -> u8 {
    while eeprom.eecr.read().eepe().bit_is_set() {}

    eeprom.eearl.write(|w| unsafe { w.bits(address) });

    eeprom.eecr.modify(|_, w| w.eere().set_bit());

    eeprom.eedr.read().bits()
}
