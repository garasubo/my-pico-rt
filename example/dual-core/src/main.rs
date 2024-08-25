#![no_std]
#![no_main]

extern crate alloc;

use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, systick, uart};
use pico_rt::{boot_core1, entry};

entry!(main);

pub extern "C" fn MainCore1Func() {
    let resets = Resets::new();
    let gpio = Gpio::new();
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

pub fn main() -> ! {
    let resets = Resets::new();
    let gpio = Gpio::new();
    gpio.wait_gpio_reset_done(&resets);
    gpio.set_output_enable(6);
    boot_core1();
    gpio.set_high(6);
    let mut flag = true;
    systick::init(1000 * 1000);
    loop {
        while !systick::check_counted() {}
        if flag {
            gpio.set_low(6);
            flag = false;
        } else {
            gpio.set_high(6);
            flag = true;
        }
    }
}
