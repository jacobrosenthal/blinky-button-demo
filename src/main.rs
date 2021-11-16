#![no_main]
#![no_std]

use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    let p = embassy_nrf::init(Default::default());

    let button = Input::new(p.P0_11, Pull::Up);
    let mut led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);

    rprintln!("Blinky button demo starting");
    loop {
        if button.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
