#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use m48_robo_rust::{delay_ms, hal::port::*, prelude::*};

static mut LEDS: Option<[Pin<mode::Output>; 7]> = None;
static mut NUM: u8 = 0;

const SEG_A: u8 = 1u8 << 0;
const SEG_B: u8 = 1u8 << 1;
const SEG_C: u8 = 1u8 << 4;
const SEG_D: u8 = 1u8 << 3;
const SEG_E: u8 = 1u8 << 2;
const SEG_F: u8 = 1u8 << 5;
const SEG_G: u8 = 1u8 << 6;

const NUMS: [u8; 10] = [
    SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F, // 0
    SEG_B | SEG_C,                                 // 1
    SEG_A | SEG_B | SEG_G | SEG_E | SEG_D,         // 2
    SEG_A | SEG_B | SEG_G | SEG_C | SEG_D,         // 3
    SEG_F | SEG_G | SEG_B | SEG_C,                 // 4
    SEG_A | SEG_F | SEG_G | SEG_C | SEG_D,         // 5
    SEG_F | SEG_G | SEG_C | SEG_D | SEG_E,         // 6
    SEG_A | SEG_B | SEG_C,                         // 7
    SEG_A | SEG_B | SEG_C | SEG_D | SEG_E | SEG_F | SEG_G, // 8
    SEG_F | SEG_A | SEG_B | SEG_G | SEG_C,         // 9
];

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portd = dp.PORTD.split();
    let mut portc = dp.PORTC.split();

    let pc0 = portc.pc0.into_output(&mut portc.ddr);
    let pc1 = portc.pc1.into_output(&mut portc.ddr);
    let pc2 = portc.pc2.into_output(&mut portc.ddr);
    let pc3 = portc.pc3.into_output(&mut portc.ddr);
    let pc4 = portc.pc4.into_output(&mut portc.ddr);
    let pd3 = portd.pd3.into_output(&mut portd.ddr);
    let pd4 = portd.pd4.into_output(&mut portd.ddr);

    unsafe {
        LEDS = Some([
            pc0.downgrade(),
            pc1.downgrade(),
            pc2.downgrade(),
            pc3.downgrade(),
            pc4.downgrade(),
            pd3.downgrade(),
            pd4.downgrade(),
        ]);

        let leds = LEDS.as_mut().unwrap();

        for i in 0..leds.len() {
            leds[i].set_high().void_unwrap();

            leds[if i < 1 { leds.len() - 1 } else { i - 1 }]
                .set_low()
                .void_unwrap();

            delay_ms(250);
        }
        leds[leds.len() - 1].set_low().void_unwrap();
    }

    let tc1 = dp.TC1;

    tc1.ocr1a.write(|w| unsafe { w.bits(15625) });
    tc1.timsk1.write(|w| w.ocie1a().set_bit());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b1u8));
    tc1.tccr1b.modify(|_, w| w.cs1().prescale_64());

    unsafe {
        // Enable interrupts
        avr_device::interrupt::enable();
    }

    loop {}
}

#[avr_device::interrupt(atmega48p)]
unsafe fn TIMER1_COMPA() {
    apply_segments(LEDS.as_mut().unwrap(), NUMS[NUM as usize]);
    NUM = if NUM < 9 { NUM + 1 } else { 0 };
}

fn apply_segments(leds: &mut [Pin<mode::Output>; 7], segments: u8) {
    for i in 0..leds.len() {
        if segments & 1u8 << i != 0 {
            leds[i].set_high().void_unwrap();
        } else {
            leds[i].set_low().void_unwrap();
        }
    }
}
