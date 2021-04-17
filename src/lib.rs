#![no_std]
#![allow(non_camel_case_types)]

pub use hal::pac;
pub use hal::pac::Peripherals;
pub use nrf51_hal as hal;

pub use nb::*;

pub mod display;
pub mod led;
pub mod gpio;

#[macro_export]
macro_rules! serial_port {
    ( $gpio:expr, $uart:expr, $speed:expr ) => {{
        use microbit::hal::{gpio::Level, uart};

        /* Configure RX and TX pins accordingly */
        let pins = uart::Pins {
            rxd: $gpio.p0_25.into_floating_input().degrade(),
            txd: $gpio.p0_24.into_push_pull_output(Level::Low).degrade(),
            cts: None,
            rts: None,
        };

        /* Set up serial port using the prepared pins */
        uart::Uart::new($uart, pins, uart::Parity::EXCLUDED, $speed)
    }};
}
