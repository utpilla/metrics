use lazy_static::lazy_static;
use metrics::counter::Counter;
use rand::{rngs::SmallRng, Rng, SeedableRng};

mod throughput;

lazy_static! {
    static ref COUNTER: Counter = Counter::new_with_periodic_flush();
    static ref ATTRIBUTE_VALUES: [&'static str; 10] = [
        "value1", "value2", "value3", "value4", "value5", "value6", "value7", "value8", "value9",
        "value10"
    ];
}

fn main() {
    throughput::test_throughput(counter);
}

fn counter() {
    let mut rng = SmallRng::from_entropy();
    let len = ATTRIBUTE_VALUES.len();
    let index_first_attribute = rng.gen_range(0..len);
    let index_second_attribute = rng.gen_range(0..len);
    let index_third_attribute = rng.gen_range(0..len);

    let attributes3 = [
        ("key1", ATTRIBUTE_VALUES[index_first_attribute]),
        ("key2", ATTRIBUTE_VALUES[index_second_attribute]),
        ("key3", ATTRIBUTE_VALUES[index_third_attribute]),
    ];

    COUNTER.add("test", &attributes3);
}

