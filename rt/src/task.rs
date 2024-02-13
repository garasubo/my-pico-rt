use core::{future::Future, pin::Pin};

struct Task {
    future: dyn Future<Output = ()>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Task {
            future,
        }
    }
}