use alloc::boxed::Box;
use core::{future::Future, pin::Pin, ptr};
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use util::linked_list::{LinkedList, ListItem};

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

pub struct SimpleExecutor<'a> {
    task_queue: LinkedList<'a, Task>,
}

impl<'a> SimpleExecutor<'a> {
    pub fn new() -> SimpleExecutor<'a> {
        SimpleExecutor {
            task_queue: LinkedList::new(),
        }
    }

    pub fn spawn(&mut self, item: &'a mut ListItem<'a, Task>) {
        self.task_queue.push(item);
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {}
                Poll::Pending => self.task_queue.push(task),
            }
        }
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(ptr::null(), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}