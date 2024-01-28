use core::ops::Deref;
use volatile_register::{RO, RW};

const RESET_BASE: usize = 0x4000_c000;

#[repr(C)]
pub struct ResetRegisters {
    pub reset: RW<u32>,
    pub wdsel: RW<u32>,
    pub reset_done: RO<u32>,
}

pub struct Resets;

impl Deref for Resets {
    type Target = ResetRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(RESET_BASE as *const ResetRegisters) }
    }
}

impl Resets {
    pub fn new() -> Self {
        Self
    }
}
