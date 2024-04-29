use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::{
    collections::HashMap,
    sync::{atomic::{AtomicU64, AtomicUsize}, RwLock},
};

use ahash::{AHasher, RandomState};

#[derive(Clone)]
pub struct MetricPoint {
    inner: Arc<MetricPointInner>,
}

pub struct MetricPointInner {
    sum: AtomicU64,
}

impl MetricPointInner {
    fn new() -> MetricPointInner {
        MetricPointInner {
            sum: AtomicU64::new(1),
        }
    }
}

impl MetricPoint {
    pub fn new() -> MetricPoint {
        MetricPoint {
            inner: Arc::new(MetricPointInner::new()),
        }
    }

    pub fn add(&self, value: u32) {
        self.inner.sum
            .fetch_add(value as u64, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_sum(&self) -> u32 {
        self.inner.sum.load(std::sync::atomic::Ordering::Relaxed) as u32
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct MetricAttributes {
    attributes: Vec<(&'static str, &'static str)>,
    hash_value: u64,
}

impl MetricAttributes {
    fn new(attributes: &[(&'static str, &'static str)]) -> MetricAttributes {
        let attributes_vec = attributes.to_vec();
        let hash_value = calculate_hash(&attributes_vec);
        MetricAttributes {
            attributes: attributes_vec,
            hash_value: hash_value,
        }
    }

    fn new_from_vec(attributes: Vec<(&'static str, &'static str)>) -> MetricAttributes {
        let hash_value = calculate_hash(&attributes);
        MetricAttributes {
            attributes,
            hash_value: hash_value,
        }
    }
}

impl Hash for MetricAttributes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash_value)
    }
}

fn calculate_hash(values: &[(&'static str, &'static str)]) -> u64 {
    let mut hasher = AHasher::default();
    values.iter().fold(&mut hasher, |mut hasher, item| {
        item.hash(&mut hasher);
        hasher
    });
    hasher.finish()
}

#[derive(Clone)]
pub struct Counter {
    inner : Arc<CounterInner>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            inner: Arc::new(CounterInner::new()),
        }
    }

    pub fn new_with_periodic_flush() -> Counter {
        let counter = Counter::new();
        let counter_clone = counter.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(10));
                counter_clone.cleanup();
            }
        });
        counter
    }

    pub fn add(&self, name: &'static str, attributes: &[(&'static str, &'static str)]) {
        self.inner.add(name, attributes);
    }

    pub fn display_metrics(&self) {
        self.inner.display_metrics();
    }

    pub fn cleanup(&self) {
        self.inner.cleanup();
    }
}

pub struct CounterInner {
    metric_points_map: RwLock<HashMap<MetricAttributes, MetricPoint, RandomState>>,
    zero_attribute_point : AtomicUsize,
}

impl CounterInner {
    pub fn new() -> CounterInner {
        let counter = CounterInner {
            metric_points_map: RwLock::new(HashMap::default()),
            zero_attribute_point: AtomicUsize::new(0),
        };
        counter
    }

    pub fn cleanup(&self) {
        self.metric_points_map.write().unwrap().clear();
        self.zero_attribute_point.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn add(&self, _name: &'static str, attributes: &[(&'static str, &'static str)]) {
        if attributes.is_empty() {
            self.zero_attribute_point.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return;
        }

        let metric_attributes = MetricAttributes::new(attributes);
        let metric_points_map = self.metric_points_map.read().unwrap();
        if let Some(metric_point) = metric_points_map.get(&metric_attributes) {
            metric_point.add(1);
        } else {
            drop(metric_points_map);
            let mut metric_points_map = self.metric_points_map.write().unwrap();
            // sort and try again
            let mut attributes_as_vec = attributes.to_vec();
            attributes_as_vec.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
            let metric_attributes_sorted = MetricAttributes::new_from_vec(attributes_as_vec);

            if let Some(metric_point) = metric_points_map.get(&metric_attributes_sorted) {
                metric_point.add(1);
            } else {
                // insert both incoming order and sorted order
                // insert in incoming order.
                let mp_new = MetricPoint::new();
                metric_points_map.insert(metric_attributes, mp_new.clone());

                // insert in sorted order
                metric_points_map.insert(metric_attributes_sorted.clone(), mp_new);
            }
        }
    }

    pub fn display_metrics(&self) {
        println!("Metrics:");
        let metric_points_map = self.metric_points_map.read().unwrap();
        for metric_point in metric_points_map.iter() {
            println!(
                "Attributes: {:?} Sum: {}",
                metric_point.0.attributes,
                metric_point.1.get_sum(),
            );
        }

        println!(
            "Zero attribute point: {}",
            self.zero_attribute_point.load(std::sync::atomic::Ordering::Relaxed)
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
