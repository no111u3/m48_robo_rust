#![no_std]
#![no_main]

extern crate panic_halt;

use m48_robo_rust::{delay_ms, prelude::*, spi};

use embedded_hal::spi::*;
use ssd1306::prelude::*;
use ssd1306::{
    command::{AddrMode, Command, VcomhLevel},
    displaysize::{DisplaySize, DisplaySize128x64},
};

use display_interface::DataFormat::U8;

#[m48_robo_rust::entry]
fn main() -> ! {
    let dp = m48_robo_rust::Peripherals::take().unwrap();

    let mut pinsd = dp.PORTD.split();

    let mut pinsb = dp.PORTB.split();

    let mut serial = m48_robo_rust::Serial::new(
        dp.USART0,
        pinsd.pd0,
        pinsd.pd1.into_output(&mut pinsd.ddr),
        2400,
    );

    pinsb.pb2.into_output(&mut pinsb.ddr); // SS must be set to output mode.

    let spi = spi::Spi::new(
        dp.SPI,
        pinsb.pb5.into_output(&mut pinsb.ddr),
        pinsb.pb3.into_output(&mut pinsb.ddr),
        pinsb.pb4.into_pull_up_input(&mut pinsb.ddr),
        spi::Settings {
            data_order: spi::DataOrder::MostSignificantFirst,
            clock: spi::SerialClockRate::OscfOver2,
            mode: Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
        },
    );

    let dc = pinsb.pb0.into_output(&mut pinsb.ddr);
    let cs = pinsb.pb1.into_output(&mut pinsb.ddr);
    let mut reset = pinsb.pb6.into_output(&mut pinsb.ddr);

    ufmt::uwriteln!(&mut serial, "SSD1306 spi from ATmega48P!\r").void_unwrap();

    let mut iface = SPIInterface::new(spi, dc, cs);

    reset.set_high().unwrap();
    delay_ms(1);
    reset.set_low().unwrap();
    delay_ms(10);
    reset.set_high().unwrap();

    Command::DisplayOn(false).send(&mut iface).unwrap();
    Command::DisplayClockDiv(0x8, 0x0).send(&mut iface).unwrap();
    Command::Multiplex(DisplaySize128x64::HEIGHT - 1)
        .send(&mut iface)
        .unwrap();
    Command::DisplayOffset(0).send(&mut iface).unwrap();
    Command::StartLine(0).send(&mut iface).unwrap();
    // TODO: Ability to turn charge pump on/off
    Command::ChargePump(true).send(&mut iface).unwrap();
    Command::AddressMode(AddrMode::Horizontal)
        .send(&mut iface)
        .unwrap();

    Command::SegmentRemap(true).send(&mut iface).unwrap();
    Command::ReverseComDir(true).send(&mut iface).unwrap();

    Command::PreChargePeriod(1, 10).send(&mut iface).unwrap();
    Command::Contrast(15).send(&mut iface).unwrap();
    Command::VcomhDeselect(VcomhLevel::Auto)
        .send(&mut iface)
        .unwrap();
    Command::AllOn(false).send(&mut iface).unwrap();
    Command::Invert(false).send(&mut iface).unwrap();
    Command::EnableScroll(true).send(&mut iface).unwrap();
    Command::DisplayOn(true).send(&mut iface).unwrap();

    ufmt::uwriteln!(&mut serial, "Done!\r").void_unwrap();

    for _ in 0..128 * 64 / 8 {
        iface.send_data(U8(&[0x00])).unwrap();
    }

    for _ in 0..128 * 64 / 8 {
        iface.send_data(U8(&[0xff])).unwrap();
    }

    for x in 0..128 * 64 / 8 {
        iface.send_data(U8(&[x as u8])).unwrap();
    }

    loop {}
}
