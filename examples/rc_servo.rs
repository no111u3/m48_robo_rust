#![feature(llvm_asm)]
#![no_std]
#![no_main]

use panic_halt as _;

use m48_robo_rust::prelude::*;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let tc1 = dp.TC1;

    let set_duty = |duty| tc1.ocr1a.write(|w| unsafe { w.bits(duty) });
    tc1.icr1.write(|w| unsafe { w.bits(2500 - 1) });
    set_duty(80);
    tc1.tccr1a
        .write(|w| w.com1a().match_clear().wgm1().bits(0b10u8));
    tc1.tccr1b.write(|w| w.wgm1().bits(0b11u8));
    tc1.tccr1b.modify(|_, w| w.cs1().prescale_8());

    let _pin = portb.pb1.into_output(&mut portb.ddr);

    loop {
        for i in 80..=267u16 {
            set_duty(i);
            m48_robo_rust::delay_ms(10);
        }
        for i in (80..=267u16).rev() {
            set_duty(i);
            m48_robo_rust::delay_ms(10);
        }
    }
}
