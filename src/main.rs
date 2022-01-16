#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy::util::Forever;
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pull};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rtt_target::{rprintln, rtt_init_print};

static EXECUTOR: Forever<embassy::executor::Executor> = Forever::new();

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();

    // this call is what overflows when setting a breakpoint
    let _executor = EXECUTOR.put(embassy::executor::Executor::new());
    let p = embassy_nrf::init(Default::default());

    let button = Input::new(p.P1_00, Pull::Up);
    let mut led = Output::new(p.P0_23, Level::Low, OutputDrive::Standard);

    rprintln!("Blinky button demo starting");
    loop {
        if button.is_high().unwrap() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
