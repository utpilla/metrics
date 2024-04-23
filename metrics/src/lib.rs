use std::hash::{Hash, Hasher};
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, RwLock},
};

use std::collections::hash_map::DefaultHasher;

pub struct MetricPoint {
    name: &'static str,
    sum: AtomicU64,
    attributes: Vec<(&'static str, &'static str)>,
}

impl MetricPoint {
    pub fn new(name: &'static str, attributes: Vec<(&'static str, &'static str)>) -> MetricPoint {
        MetricPoint {
            name,
            sum: AtomicU64::new(1),
            attributes,
        }
    }

    pub fn add(&self, value: u32) {
        self.sum
            .fetch_add(value as u64, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_sum(&self) -> u32 {
        self.sum.load(std::sync::atomic::Ordering::Relaxed) as u32
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
    let mut hasher = DefaultHasher::new();
    values.iter().fold(&mut hasher, |mut hasher, item| {
        item.hash(&mut hasher);
        hasher
    });
    hasher.finish()
}

pub struct Counter {
    metric_points_map: RwLock<HashMap<MetricAttributes, usize>>,
    metric_points: RwLock<Vec<MetricPoint>>,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            metric_points_map: RwLock::new(HashMap::new()),
            metric_points: RwLock::new(Vec::new()),
        }
    }

    pub fn add(&self, name: &'static str, attributes: &[(&'static str, &'static str)]) {
        let metric_attributes = MetricAttributes::new(attributes);
        let metric_points_map = self.metric_points_map.read().unwrap();
        if let Some(&index) = metric_points_map.get(&metric_attributes) {
            let metric_points = self.metric_points.read().unwrap();
            metric_points[index].add(1);
        } else {
            drop(metric_points_map);
            let mut metric_points_map = self.metric_points_map.write().unwrap();
            let mut metric_points = self.metric_points.write().unwrap();
            // sort and try again
            let mut attributes_as_vec = attributes.to_vec();
            attributes_as_vec.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
            let metric_attributes_sorted = MetricAttributes::new_from_vec(attributes_as_vec);

            if let Some(&index) = metric_points_map.get(&metric_attributes_sorted) {
                metric_points[index].add(1);
            } else {
                // insert both incoming order and sorted order
                let index = metric_points.len();

                // insert in incoming order.
                metric_points_map.insert(metric_attributes, index);

                // insert in sorted order
                metric_points_map.insert(metric_attributes_sorted.clone(), index);
                metric_points.push(MetricPoint::new(name, metric_attributes_sorted.attributes));
            }
        }
    }

    pub fn display_metrics(&self) {
        println!("Metrics:");
        let metric_points = self.metric_points.read().unwrap();
        for metric_point in metric_points.iter() {
            println!(
                "Name: {}, Attributes: {:?} Sum: {}",
                metric_point.name,
                metric_point.attributes,
                metric_point.get_sum()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
