pub struct Solution {}

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut max_heap = std::collections::BinaryHeap::<i32>::new();
        for s in stones {
            max_heap.push(s);
        }

        while max_heap.len() > 1 {
            let first = max_heap.pop().unwrap();
            let second = max_heap.pop().unwrap();

            let diff = i32::abs(first - second);
            if diff != 0 {
                max_heap.push(diff)
            }
        }

        *max_heap.peek().unwrap_or(&0)
    }
}
