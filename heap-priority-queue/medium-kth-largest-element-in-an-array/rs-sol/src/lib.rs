pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        type Elem = std::cmp::Reverse<i32>;

        let mut min_heap = std::collections::BinaryHeap::<Elem>::new();

        for n in nums {
            if min_heap.len() < k as usize {
                min_heap.push(std::cmp::Reverse(n));
                continue;
            }
            if n <= min_heap.peek().unwrap().0 {
                continue;
            }

            min_heap.push(std::cmp::Reverse(n));
            min_heap.pop();
        }

        min_heap.pop().unwrap().0
    }
}
