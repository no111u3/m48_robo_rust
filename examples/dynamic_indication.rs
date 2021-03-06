#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use m48_robo_rust::{delay_ms, hal::port::*, prelude::*};

static mut LEDS: Option<[Pin<mode::Output>; 8]> = None;
static mut DIGITS: Option<[Pin<mode::Output>; 3]> = None;
static mut BUFF: [u8; 3] = [0, 0, 0];

const SEG_A: u8 = 1u8 << 0;
const SEG_B: u8 = 1u8 << 1;
const SEG_C: u8 = 1u8 << 2;
const SEG_D: u8 = 1u8 << 3;
const SEG_E: u8 = 1u8 << 4;
const SEG_F: u8 = 1u8 << 5;
const SEG_G: u8 = 1u8 << 6;
const SEG_DP: u8 = 1u8 << 7;

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

    unsafe {
        LEDS = Some([
            portc.pc0.into_output(&mut portc.ddr).downgrade(),
            portc.pc1.into_output(&mut portc.ddr).downgrade(),
            portc.pc2.into_output(&mut portc.ddr).downgrade(),
            portc.pc3.into_output(&mut portc.ddr).downgrade(),
            portc.pc4.into_output(&mut portc.ddr).downgrade(),
            portd.pd3.into_output(&mut portd.ddr).downgrade(),
            portd.pd4.into_output(&mut portd.ddr).downgrade(),
            portd.pd2.into_output(&mut portd.ddr).downgrade(),
        ]);

        DIGITS = Some([
            portd.pd5.into_output(&mut portd.ddr).downgrade(),
            portd.pd6.into_output(&mut portd.ddr).downgrade(),
            portd.pd7.into_output(&mut portd.ddr).downgrade(),
        ]);

        let leds = LEDS.as_mut().unwrap();

        let digits = DIGITS.as_mut().unwrap();

        for i in 0..digits.len() {
            digits[i].set_high().void_unwrap();
            digits[if i < 1 { digits.len() - 1 } else { i - 1 }]
                .set_low()
                .void_unwrap();

            for i in 0..leds.len() {
                leds[i].set_high().void_unwrap();

                leds[if i < 1 { leds.len() - 1 } else { i - 1 }]
                    .set_low()
                    .void_unwrap();

                delay_ms(250);
            }
            leds[leds.len() - 1].set_low().void_unwrap();
        }
    }

    let tc1 = dp.TC1;

    tc1.ocr1a.write(|w| unsafe { w.bits(6666) });
    tc1.timsk1.write(|w| w.ocie1a().set_bit());
    tc1.tccr1b.write(|w| w.wgm1().bits(0b1u8));
    tc1.tccr1b.modify(|_, w| w.cs1().direct());

    unsafe {
        // Enable interrupts
        avr_device::interrupt::enable();
    }

    unsafe {
        BUFF[0] = NUMS[0];
        BUFF[1] = NUMS[1];
        BUFF[2] = NUMS[2];
    }

    let mut num = 0;

    delay_ms(500);

    loop {
        delay_ms(250);
        unsafe {
            BUFF[0] = NUMS[num / 100];
            BUFF[1] = NUMS[num / 10 % 10];
            BUFF[2] = NUMS[num % 10];

            BUFF[2] ^= SEG_DP;
        }
        num = if num < 999 { num + 1 } else { 0 }
    }
}

#[avr_device::interrupt(atmega48p)]
unsafe fn TIMER1_COMPA() {
    static mut CURRENT: u8 = 0;
    let digits = DIGITS.as_mut().unwrap();
    let num_repr = match CURRENT {
        0 => {
            digits[2].set_low().void_unwrap();
            digits[0].set_high().void_unwrap();
            CURRENT = 1;
            BUFF[0]
        }
        1 => {
            digits[0].set_low().void_unwrap();
            digits[1].set_high().void_unwrap();
            CURRENT = 2;
            BUFF[1]
        }
        2 => {
            digits[1].set_low().void_unwrap();
            digits[2].set_high().void_unwrap();
            CURRENT = 0;
            BUFF[2]
        }
        _ => unreachable!(),
    };
    apply_segments(num_repr);
}

#[inline(always)]
fn apply_segments(segments: u8) {
    let leds = unsafe { LEDS.as_mut().unwrap() };
    for i in 0..leds.len() {
        if segments & (1u8 << i) != 0 {
            leds[i].set_high().void_unwrap();
        } else {
            leds[i].set_low().void_unwrap();
        }
    }
}
