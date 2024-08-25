#![no_std]
#![no_main]

use core::arch::asm;
use core::sync::atomic::AtomicBool;
use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, systick, uart};
use pico_rt::{entry, systick_entry};

entry!(main);
systick_entry!(systick_handler);

pub fn systick_handler() {
    static FLAG: AtomicBool = AtomicBool::new(true);
    let gpio = Gpio::new();
    if FLAG.load(core::sync::atomic::Ordering::Relaxed) {
        gpio.set_low(6);
        FLAG.store(false, core::sync::atomic::Ordering::Relaxed);
    } else {
        gpio.set_high(6);
        FLAG.store(true, core::sync::atomic::Ordering::Relaxed);
    }
}

pub fn main() -> ! {
    let resets = Resets::new();
    let gpio = Gpio::new();
    gpio.wait_gpio_reset_done(&resets);
    gpio.set_output_enable(6);
    gpio.set_high(6);
    systick::init(1000 * 1000);
    systick::enable_interrupt();

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
