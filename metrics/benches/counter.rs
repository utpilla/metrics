use criterion::{criterion_group, criterion_main, Criterion};
use metrics::Counter;

// cargo bench --bench counter
pub fn counter_benchmark(c: &mut Criterion) {
    let tracker = Counter::new_with_periodic_flush();

    let attributes1 = [("key1", "value1")];

    let attributes3 = [("key1", "value1"), ("key2", "value2"), ("key3", "value3")];

    let attributes5 = [
        ("key1", "value1"),
        ("key2", "value2"),
        ("key3", "value3"),
        ("key4", "value4"),
        ("key5", "value5"),
    ];

    c.bench_function("counter_0", |b| {
        b.iter(|| {
            tracker.add("counter", &[]);
        });
    });

    c.bench_function("counter_1", |b| {
        b.iter(|| {
            tracker.add("counter", &attributes1);
        });
    });

    c.bench_function("counter_3", |b| {
        b.iter(|| {
            tracker.add("counter", &attributes3);
        });
    });

    c.bench_function("counter_5", |b| {
        b.iter(|| {
            tracker.add("counter", &attributes5);
        });
    });

    tracker.display_metrics();
}

criterion_group!(benches, counter_benchmark);
criterion_main!(benches);
