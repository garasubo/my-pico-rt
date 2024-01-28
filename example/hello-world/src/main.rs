#![no_std]
#![no_main]

use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, uart};
use pico_rt::entry;

entry!(main);

pub fn main() -> ! {
    let resets = Resets::new();
    let gpio = Gpio::new();
    gpio.wait_gpio_reset_done(&resets);
    gpio.set_output_enable(6);
    gpio.set_high(6);

    let uart = uart::Uart0::new();
    let xosc = clock::Xosc::new();
    uart.init(&resets, &xosc, &gpio);
    for c in b"hello\n" {
        uart.putc(*c);
    }
    loop {
        let c = uart.getc();
        uart.putc(c);
    }
}
