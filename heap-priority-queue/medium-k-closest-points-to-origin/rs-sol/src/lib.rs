use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution {}

pub struct Point {
    pub x: i32,
    pub y: i32,
    pub distance_to_origin: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.distance_to_origin == other.distance_to_origin
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_to_origin.total_cmp(&other.distance_to_origin)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point {}

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut min_heap: BinaryHeap<Reverse<Point>> = BinaryHeap::new();

        for p in points.iter() {
            let dist = f64::sqrt(f64::powi(p[0] as f64, 2) + f64::powi(p[1] as f64, 2));

            min_heap.push(Reverse(Point {
                x: p[0],
                y: p[1],
                distance_to_origin: dist,
            }))
        }

        let mut resp = Vec::new();

        for _ in 0..k {
            let point = min_heap.pop().unwrap();
            resp.push(vec![point.0.x, point.0.y]);
        }

        resp
    }
}
