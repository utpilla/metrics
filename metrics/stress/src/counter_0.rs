use lazy_static::lazy_static;
use metrics::Counter;

mod throughput;

lazy_static! {
    static ref COUNTER: Counter = Counter::new_with_cleanup();
}

fn main() {
    throughput::test_throughput(counter);
}

fn counter() {
    COUNTER.add("test", &vec![]);
}

