#![no_std]
#![no_main]

extern crate alloc;

use pico_hal::gpio::Gpio;
use pico_hal::resets::Resets;
use pico_rt::entry;
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

pub fn main() -> ! {
    let resets = Resets::new();
    let gpio = Gpio::new();
    gpio.wait_gpio_reset_done(&resets);
    gpio.set_output_enable(6);

    let mut executor = SimpleExecutor::new();
    let task = Task::new(example_task(gpio));
    let mut item = ListItem::create(task);
    executor.spawn(&mut item);
    executor.run();

    loop {}
}
