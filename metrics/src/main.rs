use metrics::Counter;

fn main() {
    let counter = Counter::new();
    let attributes = [("key2", "value2"), ("key1", "value1"), ("key3", "value3")];
    let attributes_in_diff_order = [("key1", "value1"), ("key2", "value2"), ("key3", "value3")];
    let attributes_in_diff_order2 = [("key1", "value1"), ("key3", "value3"), ("key2", "value2")];

    counter.add("counter", &attributes);
    counter.add("counter", &attributes_in_diff_order);
    counter.add("counter", &attributes_in_diff_order2);
    counter.add("counter", &attributes);
    counter.add("counter", &attributes_in_diff_order);
    counter.display_metrics();
}
