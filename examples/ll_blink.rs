#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::dev::gpio;

#[m48_robo_rust::entry]
fn main() -> ! {
    gpio::GPIOB.ddrb.modify(gpio::DDRB::DDB0::SET);

    loop {
        gpio::GPIOB.portb.modify(gpio::PORTB::PORTB0::SET);

        m48_robo_rust::delay_ms(200);

        gpio::GPIOB.portb.modify(gpio::PORTB::PORTB0::CLEAR);

        m48_robo_rust::delay_ms(200);
    }
}
