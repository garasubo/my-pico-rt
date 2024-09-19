#[repr(C)]
pub struct ContextFrame {
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,
    pub lr: u32,
    pub return_addr: u32,
    pub xpsr: u32,
}

fn call_process(func: fn(), sp: u32) {
    let ptr = sp + 1024 - core::mem::size_of::<ContextFrame>() as u32;
    let context_frame = unsafe { &mut *(ptr as *mut ContextFrame) };

    context_frame.r0 = 0;
    context_frame.r1 = 0;
    context_frame.r2 = 0;
    context_frame.r3 = 0;
    context_frame.r12 = 0;
    context_frame.lr = 0;
    context_frame.return_addr = func as u32;
    context_frame.xpsr = 0x100_0000;
}
