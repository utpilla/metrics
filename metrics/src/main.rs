use std::thread;

use metrics::counter::Counter;

fn main() {
    let counter = Counter::new_with_periodic_flush();
    let attributes = [("key2", "value2"), ("key1", "value1"), ("key3", "value3")];
    let attributes_in_diff_order = [("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
    let attributes_in_diff_order2 = [("key1", "value1"), ("key3", "value3"), ("key2", "value2")];

    while (true)
    {
        counter.add("counter", &attributes);
        counter.add("counter", &attributes_in_diff_order);
        counter.add("counter", &attributes_in_diff_order2);
        counter.add("counter", &attributes);
        counter.add("counter", &attributes_in_diff_order);

        counter.add("counter2", &vec![]);
        counter.add("counter2", &vec![]);
        thread::sleep(std::time::Duration::from_secs(1));
        counter.display_metrics();
    }
}
