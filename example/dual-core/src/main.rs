#![no_std]
#![no_main]

extern crate alloc;

use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, uart};
use pico_rt::{boot_core1, entry};
use pico_rt::task::{SimpleExecutor, Task};
use util::linked_list::ListItem;

entry!(main);

async fn async_test() -> bool {
    true
}

async fn example_task(gpio: Gpio) {
    let result = async_test().await;
    if result {
        gpio.set_high(6);
    }
}

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
    loop {
    }
}
