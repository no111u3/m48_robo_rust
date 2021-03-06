#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

use m48_robo_rust::{hal::port, prelude::*, pwm};

static mut PIN: Option<port::portb::PB1<port::mode::Pwm<pwm::Timer1Pwm>>> = None;
static mut DIRECTION: bool = false;

#[avr_device::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut portb = dp.PORTB.split();

    let tc1 = dp.TC1;

    tc1.timsk1.write(|w| w.toie1().set_bit());

    let mut timer = pwm::Timer1Pwm::new(tc1, pwm::Prescaler::Prescale8);

    let mut pin = portb.pb1.into_output(&mut portb.ddr).into_pwm(&mut timer);

    pin.set_duty(128);
    pin.enable();

    unsafe {
        PIN = Some(pin);
        // Enable interrupts
        avr_device::interrupt::enable();
    }

    loop {}
}

#[avr_device::interrupt(atmega48p)]
unsafe fn TIMER1_OVF() {
    let mut duty = PIN.as_mut().unwrap().get_duty();
    if DIRECTION {
        if duty < 255 {
            duty += 1;
        } else {
            DIRECTION = false;
        }
    } else {
        if duty > 0 {
            duty -= 1;
        } else {
            DIRECTION = true;
        }
    }
    PIN.as_mut().unwrap().set_duty(duty);
}
