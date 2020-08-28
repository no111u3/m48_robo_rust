#![no_std]
#![no_main]

use m48_robo_rust::{hal::port::mode, prelude::*};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial: m48_robo_rust::Serial<mode::Floating> =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();

    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    loop {}
}

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

    ufmt::uwriteln!(&mut serial, "Panic example from ATmega48P!\r").void_unwrap();

    // Panic messages cannot yet be captured because they rely on core::fmt
    // which is way too big for AVR
    panic!();
}
