#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use atmega48p_hal::{atmega48p::Peripherals, clock, port::*, prelude::*, usart};

type Serial<IMODE> = usart::Usart0<clock::MHz1, IMODE>;

static mut SERIAL: Option<Serial<mode::Floating>> = None;

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

    serial.interrupt_rxc(true);

    unsafe {
        SERIAL = Some(serial);
        // Enable interrupts
        avr_device::interrupt::enable();
    }

    unsafe {
        ufmt::uwriteln!(
            &mut SERIAL.as_mut().unwrap(),
            "Echo interrupt from ATmega48P!\r"
        )
        .void_unwrap();
    }

    loop {}
}

#[avr_device::interrupt(atmega48p)]
unsafe fn USART_RX() {
    // Read a byte from the serial connection
    let b = nb::block!(SERIAL.as_mut().unwrap().read()).void_unwrap();

    // Answer
    nb::block!(SERIAL.as_mut().unwrap().write(b)).void_unwrap();
}
