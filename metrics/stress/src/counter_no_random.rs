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
    
    let attributes3 = [
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3"),
    ];

    COUNTER.add("test", &attributes3);
}

