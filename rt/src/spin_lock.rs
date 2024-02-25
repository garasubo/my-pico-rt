use lock_api::{GuardSend, RawMutex};
use pico_hal::sio::SioSpinLock;

pub struct PiSpinLock<const N: usize> {
    lock: SioSpinLock<N>,
}

impl <const N: usize> PiSpinLock<N> {
    pub const fn new() -> Self {
        Self {
            lock: SioSpinLock::<N>::new(),
        }
    }
}

unsafe impl <const N: usize> RawMutex for PiSpinLock<N> {
    const INIT: Self = Self {
        lock: SioSpinLock::<N>::new(),
    };
    type GuardMarker = GuardSend;

    fn lock(&self) {
        while !self.lock.lock() {}
    }

    fn try_lock(&self) -> bool {
        self.lock.lock()
    }

    unsafe fn unlock(&self) {
        self.lock.unlock();
    }
}

pub type PiSpinLockMutex<T, const N: usize> = lock_api::Mutex<PiSpinLock<N>, T>;
pub type PiSpinLockMutexGuard<'a, T, const N: usize> = lock_api::MutexGuard<'a, PiSpinLock<N>, T>;