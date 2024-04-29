use lazy_static::lazy_static;
use std::sync::atomic::{AtomicU64, Ordering};

mod throughput;

struct AtomicCounter {
    sum: AtomicU64,
}


impl AtomicCounter {
    fn new() -> Self {
        AtomicCounter {
            sum: AtomicU64::new(0),
        }
    }

    fn add(&self) {
        self.sum.fetch_add(1, Ordering::Relaxed);
    }
}

lazy_static! {
    static ref COUNTER: AtomicCounter = AtomicCounter::new();
}

fn main() {
    throughput::test_throughput(atomic);
}

fn atomic() {
    COUNTER.add();
}

