#![cfg_attr(not(test), no_std)]
#![feature(naked_functions)]

extern crate alloc;

mod allocator;
pub mod process;
mod spin_lock;
pub mod task;

use crate::allocator::LockedAllocator;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;
use core::ptr::{addr_of, addr_of_mut};

#[global_allocator]
static GLOBAL_ALLOCATOR: LockedAllocator = LockedAllocator::new();

#[link_section = ".boot_loader"]
#[used]
pub static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
        static _heap_start: u8;
    }

    let sbss = addr_of_mut!(_sbss);
    let ebss = addr_of_mut!(_ebss);
    let sdata = addr_of_mut!(_sdata);
    let edata = addr_of_mut!(_edata);
    let sidata = &_sidata as *const u8;

    let count = ebss as usize - sbss as usize;
    ptr::write_bytes(sbss, 0, count);

    let count = edata as usize - sdata as usize;
    ptr::copy_nonoverlapping(sidata, sdata, count);

    let heap_start = addr_of!(_heap_start) as usize;
    GLOBAL_ALLOCATOR
        .0
        .lock()
        .init(heap_start, heap_start + 0x2000);

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

pub unsafe fn boot_core1(core1_main_func: extern "C" fn() -> !, core1_stack: &mut [u8]) {
    extern "C" {
        static mut _reset_vector: u8;
    }
    let reset_vector = unsafe { addr_of_mut!(_reset_vector) as usize };
    let sp = unsafe { core1_stack.as_ptr() as usize + core1_stack.len() };
    // 0x1: Thumb mode
    let entry_point = core1_main_func as *const () as usize | 0x1;
    let cmds: [usize; 6] = [0, 0, 1, reset_vector, sp, entry_point];
    let sio = pico_hal::sio::Sio::new();

    let mut iter = cmds.into_iter();
    let mut current = iter.next();
    while let Some(cmd) = current {
        // always drain the READ FIFO (from core 1) before sending a 0
        if cmd == 0 {
            sio.drain_fifo();
            // execute a SEV as core 1 may be waiting for FIFO space
            unsafe {
                asm!("sev");
            }
        }

        // write 32 bit value to write FIFO
        sio.push_fifo_blocking(cmd as u32);
        // read 32 bit value from read FIFO once available
        let response = sio.read_fifo_blocking();
        if response == cmd as u32 {
            current = iter.next();
        } else {
            iter = cmds.into_iter();
            current = iter.next();
        }
    }
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
#[cfg(not(test))]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}

#[macro_export]
#[cfg(not(test))]
macro_rules! systick_entry {
    ($path:path) => {
        #[export_name = "SysTick"]
        pub unsafe fn __systick() {
            let f: fn() = $path;

            f()
        }
    };
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union Vector {
    pub reserved: u32,
    pub handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[no_mangle]
pub extern "C" fn DefaultMainCore1Func() {
    loop {}
}

#[no_mangle]
#[naked]
pub unsafe extern "C" fn SVCall() {
    asm!(
        "
        ldr r0, 100f
        cmp lr, r0
        bne 1f

        movs r0, #1
        msr CONTROL, r0
        isb,
        ldr r0, 200f
        mov lr, r0
        bx lr

    1:
        movs r0, #0
        msr CONTROL, r0
        isb
        ldr r0, 100f
        mov lr, r0
        bx lr

    .align 4
    100:
        .word 0xfffffff9
    200:
        .word 0xfffffffd
    ",
        options(noreturn),
    );
}
