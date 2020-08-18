#![feature(llvm_asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

#[avr_device::entry]
fn main() -> ! {
    loop {}
}
