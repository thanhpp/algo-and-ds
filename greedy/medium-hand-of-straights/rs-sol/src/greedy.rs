pub struct Solution {}

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        /*
        GREEDY: always starts from the minimum value -> increase to check group
        */

        let length = hand.len() as i32;

        if group_size > length || length % group_size != 0 {
            return false;
        }

        // SPACE: O(n)
        let mut count = std::collections::HashMap::<i32, i32>::new();
        let mut min_heap = std::collections::BinaryHeap::<std::cmp::Reverse<i32>>::new();

        for h in hand {
            match count.get_mut(&h) {
                None => {
                    count.insert(h, 1);
                    min_heap.push(std::cmp::Reverse(h)); // TIME: O(logn)
                }
                Some(c) => {
                    *c += 1;
                }
            }
        }
        // TIME: O(nlogn)

        // TIME: O(n)
        while let Some(&start) = min_heap.peek() {
            for i in 0..group_size {
                let next = start.0 + i;
                match count.get_mut(&next) {
                    None => {
                        return false;
                    }
                    Some(c) => {
                        *c -= 1;
                        if *c == 0 {
                            match min_heap.pop() {
                                // TIME: O(logn)
                                None => return false,
                                Some(v) => {
                                    if v.0 != next {
                                        return false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        true
    }
}

mod test {
    #[test]
    pub fn test_solution() {
        println!(
            "{}",
            crate::greedy::Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        );
    }

    #[test]
    pub fn test_solution_1() {
        println!(
            "{}",
            crate::greedy::Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3)
        );
    }
}
