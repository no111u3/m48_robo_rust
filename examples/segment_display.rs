#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::{delay_ms, hal::port::*, prelude::*};

const SEG_A: u8 = 1u8 << 0;
const SEG_B: u8 = 1u8 << 1;
const SEG_C: u8 = 1u8 << 4;
const SEG_D: u8 = 1u8 << 3;
const SEG_E: u8 = 1u8 << 2;
const SEG_F: u8 = 1u8 << 6;
const SEG_G: u8 = 1u8 << 7;
const SEG_DP: u8 = 1u8 << 5;

const NUMS: [u8; 11] = [
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
    SEG_DP,                                        // Decimal point
];

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portd = dp.PORTD.split();
    let mut portc = dp.PORTC.split();

    let button = portd.pd2.into_pull_up_input(&mut portd.ddr);

    let pc0 = portc.pc0.into_output(&mut portc.ddr);
    let pc1 = portc.pc1.into_output(&mut portc.ddr);
    let pc2 = portc.pc2.into_output(&mut portc.ddr);
    let pc3 = portc.pc3.into_output(&mut portc.ddr);
    let pc4 = portc.pc4.into_output(&mut portc.ddr);
    let pc5 = portc.pc5.into_output(&mut portc.ddr);
    let pd3 = portd.pd3.into_output(&mut portd.ddr);
    let pd4 = portd.pd4.into_output(&mut portd.ddr);

    let mut leds: [Pin<mode::Output>; 8] = [
        pc0.downgrade(),
        pc1.downgrade(),
        pc2.downgrade(),
        pc3.downgrade(),
        pc4.downgrade(),
        pc5.downgrade(),
        pd3.downgrade(),
        pd4.downgrade(),
    ];

    for i in 0..leds.len() {
        leds[i].set_high().void_unwrap();

        leds[if i < 1 { leds.len() - 1 } else { i - 1 }]
            .set_low()
            .void_unwrap();

        delay_ms(250);
    }
    leds[leds.len() - 1].set_low().void_unwrap();

    let mut button_count: u8 = 0;
    loop {
        if button.is_low().void_unwrap() {
            if button_count < 10 {
                button_count += 1;
            } else {
                for num in &NUMS {
                    apply_segments(&mut leds, *num);
                    delay_ms(500);
                }
            }
        } else {
            button_count = 0;
        }
    }
}

fn apply_segments(leds: &mut [Pin<mode::Output>; 8], segments: u8) {
    for i in 0..leds.len() {
        if segments & 1u8 << i != 0 {
            leds[i].set_high().void_unwrap();
        } else {
            leds[i].set_low().void_unwrap();
        }
    }
}
