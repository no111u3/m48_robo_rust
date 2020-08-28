#![no_std]

pub extern crate atmega48p_hal as hal;
/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use hal::entry;

//mod pins;
//pub use crate::pins::*;

pub use crate::atmega48p::Peripherals;
pub use atmega48p_hal::atmega48p;
pub use atmega48p_hal::prelude;

/// Busy-Delay
///
/// **Note**: For just delaying, using [`m48_robo_rust::delay_ms()`][delay_ms] or
/// [`m48_robo_rust::delay_us()`][delay_us] is probably the better choice.  This type is more useful
/// when an `embedded-hal` driver needs a delay implementation.
///
/// [delay_ms]: fn.delay_ms.html
/// [delay_us]: fn.delay_us.html
pub type Delay = hal::delay::Delay<hal::clock::MHz16>;

/// Wait (busy spin) for `ms` milliseconds
pub fn delay_ms(ms: u16) {
    use prelude::*;

    Delay::new().delay_ms(ms)
}

/// Wait (busy spin) for `us` microseconds
pub fn delay_us(us: u16) {
    use prelude::*;

    Delay::new().delay_us(us)
}

/// Support for the Serial Peripheral Interface
///
/// # Example
/// For a full example, see [`examples/uno-spi-feedback.rs`][ex-spi].  In short:
/// ```no_run
/// let dp = arduino_uno::Peripherals::take().unwrap();
///
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
///
/// pins.d10.into_output(&mut pins.ddr); // Chip Select must be set to output mode.
///
/// // Create SPI interface.
/// let mut spi = arduino_uno::spi::Spi::new(
///     dp.SPI,
///     pins.d13.into_output(&mut pins.ddr),
///     pins.d11.into_output(&mut pins.ddr),
///     pins.d12.into_pull_up_input(&mut pins.ddr),
///     arduino_uno::spi::Settings::default(),
/// );
/// ```
///
/// [ex-spi]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-spi-feedback.rs
pub mod spi {
    pub use atmega48p_hal::spi::*;
}

/// Support for the Analog to Digital Converter
///
/// # Example
/// For a full example, see [`examples/uno-adc.rs`][ex-adc].  In short:
/// ```no_run
/// let dp = arduino_uno::Peripherals::take().unwrap();
///
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
///
/// let mut adc = arduino_uno::adc::Adc::new(dp.ADC, arduino_uno::adc::AdcSettings::default());
///
/// // Convert pin to Analog input
/// let mut a0 = pins.a0.into_analog_input(&mut adc);
///
/// let aread: u16 = nb::block!{adc.read(&mut a0)}.void_unwrap();
/// ```
///
/// [ex-adc]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-adc.rs
pub mod adc {
    pub use atmega48p_hal::adc::*;
}

/// Support for PWM pins
///
/// The 3 timers of ATmega328P can be used for PWM on certain pins.
/// The PWM methods are from `embedded_hal::PwmPin`.
///
/// # Example
/// For a full example, see [`examples/uno-pwm.rs`][ex-pwm].  In short:
/// ```
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
///
/// let mut timer1 = arduino_uno::pwm::Timer1Pwm::new(
///     dp.TC1,
///     arduino_uno::pwm::Prescaler::Prescale64,
/// );
///
/// let mut pin = pins.d9.into_output(&mut pins.ddr).into_pwm(&mut timer1);
///
/// pin.set_duty(128);
/// pin.enable();
/// ```
///
/// Here is an overview of pins and which timer they work with:
///
/// | Pin | Conversion Method |
/// | --- | --- |
/// | `pins.d3` | `.into_pwm(&mut timer2)` |
/// | `pins.d5` | `.into_pwm(&mut timer0)` |
/// | `pins.d6` | `.into_pwm(&mut timer0)` |
/// | `pins.d9` | `.into_pwm(&mut timer1)` |
/// | `pins.d10` | `.into_pwm(&mut timer1)` |
/// | `pins.d11` | `.into_pwm(&mut timer2)` |
///
/// [ex-pwm]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-pwm.rs
pub mod pwm {
    pub use atmega48p_hal::pwm::*;
}

/// Serial (UART) interface on pins `D0` (RX) and `D1` (TX)
///
/// # Example
/// For a full example, see [`examples/leonardo-serial.rs`][ex-serial].  In short:
/// ```no_run
/// let dp = arduino_uno::Peripherals::take().unwrap();
///
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
///
/// let mut serial = arduino_uno::Serial::new(
///     dp.USART0,
///     pins.d0,
///     pins.d1.into_output(&mut pins.ddr),
///     57600,
/// );
///
/// ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
/// ```
///
/// [ex-serial]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-serial.rs
pub type Serial<IMODE> = hal::usart::Usart0<hal::clock::MHz1, IMODE>;

/// I2C Master on pins `A4` (SDA) and `A5` (SCL)
///
/// # Example
/// For a full example, see [`examples/leonardo-i2cdetect.rs`][ex-i2c].  In short:
/// ```no_run
/// let dp = arduino_uno::Peripherals::take().unwrap();
///
/// let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
///
/// let mut i2c = arduino_uno::I2c::new(
///     dp.TWI,
///     pins.a4.into_pull_up_input(&mut pins.ddr),
///     pins.a5.into_pull_up_input(&mut pins.ddr),
///     50000,
/// );
/// ```
///
/// [ex-i2c]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-i2cdetect.rs
pub type I2c<M> = hal::i2c::I2c<hal::clock::MHz1, M>;
