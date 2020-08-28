#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::prelude::*;
use micromath::F32Ext;

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

    let x = 4.5f32;

    let y = 2.9f32;

    let z = x + y * 1.5;

    ufmt::uwriteln!(
        &mut serial,
        "for x = 4.5 and y = 2.9 x + y * 1.5 = {}\r",
        UFloat(z)
    )
    .void_unwrap();

    loop {}
}

struct UFloat(f32);

use ufmt::{uDisplay, uWrite, uwrite, Formatter};

impl uDisplay for UFloat {
    fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: uWrite + ?Sized,
    {
        match self.0 {
            x if x.is_infinite() && x.is_sign_negative() => uwrite!(f, "-inf"),
            x if x.is_infinite() => uwrite!(f, "inf"),
            x if x.is_nan() => uwrite!(f, "nan"),
            x => uwrite!(f, "{}.{}", x.floor() as i32, get_frac(x)),
        }
    }
}

fn get_frac(f: f32) -> u32 {
    (f.abs().fract() * 10000.0) as u32
}
