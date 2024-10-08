use core::arch::asm;
use core::ops::Deref;
use volatile_register::{RO, RW, WO};

const SIO_BASE: usize = 0xd000_0000;

#[repr(C)]
pub struct SioRegisters {
    cpuid: RO<u32>,
    gpio_in: RO<u32>,
    gpio_hi_in: RO<u32>,
    _reserved0: u32,
    // 0x10
    gpio_out: RW<u32>,
    gpio_out_set: WO<u32>,
    gpio_out_clr: WO<u32>,
    gpio_out_xor: WO<u32>,
    // 0x20
    gpio_oe: RW<u32>,
    gpio_oe_set: WO<u32>,
    gpio_oe_clr: WO<u32>,
    gpio_oe_xor: WO<u32>,
    // 0x30
    gpio_hi_out: RW<u32>,
    gpio_hi_out_set: WO<u32>,
    gpio_hi_out_clr: WO<u32>,
    gpio_hi_out_xor: WO<u32>,
    // 0x40
    gpio_hi_oe: RW<u32>,
    gpio_hi_oe_set: WO<u32>,
    gpio_hi_oe_clr: WO<u32>,
    gpio_hi_oe_xor: WO<u32>,
    // 0x50
    fifo_st: RW<u32>,
    fifo_wr: WO<u32>,
    fifo_rd: RO<u32>,
    spinlock_st: RO<u32>,
    // 0x60
}

impl SioRegisters {
    pub fn clear_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_clr.write(0x1 << pin);
        }
    }

    pub fn clear_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_clr.write(0x1 << pin);
        }
    }

    pub fn set_gpio_oe(&self, pin: usize) {
        unsafe {
            self.gpio_oe_set.write(0x1 << pin);
        }
    }

    pub fn set_gpio_out(&self, pin: usize) {
        unsafe {
            self.gpio_out_set.write(0x1 << pin);
        }
    }
}

pub struct Sio;

impl Sio {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cpu_id(&self) -> u32 {
        self.cpuid.read()
    }

    pub fn read_fifo_blocking(&self) -> u32 {
        while (self.fifo_st.read() & 0x1) == 0 {
            unsafe {
                asm!("wfe");
            }
        }
        self.fifo_rd.read()
    }

    pub fn drain_fifo(&self) {
        while (self.fifo_st.read() & 0x1) == 1 {
            let _ = self.fifo_rd.read();
        }
    }

    pub fn push_fifo_blocking(&self, data: u32) {
        while self.fifo_st.read() & 0x2 == 0 {}
        unsafe {
            self.fifo_wr.write(data);
            asm!("sev");
        }
    }
}

impl Deref for Sio {
    type Target = SioRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(SIO_BASE as *const SioRegisters) }
    }
}

#[repr(transparent)]
pub struct SioSpinLockRegister(RW<u32>);

pub struct SioSpinLock<const N: usize>;

impl<const N: usize> Deref for SioSpinLock<N> {
    type Target = SioSpinLockRegister;

    fn deref(&self) -> &Self::Target {
        unsafe { &*((SIO_BASE + 0x100 + N * 0x4) as *const SioSpinLockRegister) }
    }
}

impl SioSpinLockRegister {
    pub fn lock(&self) -> bool {
        self.0.read() > 0
    }

    pub fn unlock(&self) {
        unsafe {
            self.0.write(0x1);
        }
    }
}

impl<const N: usize> SioSpinLock<N> {
    pub const fn new() -> Self {
        assert!(N < 32);
        Self {}
    }
}
