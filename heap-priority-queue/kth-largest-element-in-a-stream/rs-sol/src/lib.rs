use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: i32,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k = KthLargest {
            k,
            min_heap: BinaryHeap::new(),
        };

        for n in nums {
            k.add(n);
        }

        k
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.k as usize > self.min_heap.len() {
            self.min_heap.push(Reverse(val));
            return self.min_heap.peek().unwrap().0;
        }

        let p = *self.min_heap.peek().unwrap();
        if val < p.0 {
            return p.0;
        }

        self.min_heap.pop();
        self.min_heap.push(Reverse(val));

        self.min_heap.peek().unwrap().0
    }
}
