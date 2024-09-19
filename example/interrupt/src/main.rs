#![no_std]
#![no_main]

use aligned::{Aligned, A8};
use core::arch::asm;
use core::sync::atomic::AtomicBool;
use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_hal::{clock, systick, uart};
use pico_rt::{boot_core1, entry, systick_entry};

entry!(main);
systick_entry!(systick_handler);

// 1KB
static mut CORE1_STACK: Aligned<A8, [u8; 1024]> = Aligned([0; 1024]);

pub fn systick_handler() {
    let sio = pico_hal::sio::Sio::new();
    let cpu_id = sio.cpu_id();
    if cpu_id == 0 {
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
}

extern "C" fn core1_main() -> ! {
    let resets = Resets::new();
    let gpio = Gpio::new();
    let uart = uart::Uart0::new();
    let xosc = clock::Xosc::new();
    uart.init(&resets, &xosc, &gpio);
    systick::init(1000 * 1000);
    systick::enable_interrupt();
    unsafe {
        asm!("cpsie i");
    }
    loop {
        for c in b"hello\n" {
            uart.putc(*c);
        }
        unsafe {
            asm!("wfi");
        }
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
    unsafe {
        boot_core1(core1_main, CORE1_STACK.as_mut());
    }

    loop {}
}
