#![no_std]
#![no_main]

extern crate alloc;

use aligned::{Aligned, A8};
use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, systick, uart};
use pico_rt::{boot_core1, entry};

entry!(main);

// 1KB
static mut CORE1_STACK: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);

pub extern "C" fn core1_main() -> ! {
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
    unsafe { boot_core1(core1_main, CORE1_STACK.as_mut()) };
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
