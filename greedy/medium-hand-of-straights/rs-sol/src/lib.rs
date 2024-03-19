pub struct Solution {}

mod greedy;

impl Solution {
    // TIME: O(nlogn + n*group_size)
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let length = hand.len() as i32;
        if group_size == 0 {
            return true;
        }
        if length == 0 || length % group_size != 0 {
            return false;
        }

        // TIME: O(n)
        let mut count = std::collections::HashMap::<i32, i32>::new();
        for h in hand {
            match count.get_mut(&h) {
                None => {
                    count.insert(h, 1);
                }
                Some(c) => {
                    *c += 1;
                }
            }
        }

        // TIME: O(nlogn)
        let mut keys = count.keys().cloned().collect::<Vec<i32>>();
        keys.sort();

        // TIME: O(n * group_size)
        let mut key_idx = 0;
        while key_idx < keys.len() {
            let k = keys[key_idx];
            match count.get(&k) {
                None => {
                    key_idx += 1;
                    continue;
                }
                Some(c) => {
                    if *c == 0 {
                        key_idx += 1;
                        continue;
                    }
                }
            }

            for i in 0..group_size {
                println!("checking {}, {}, {}", k, i, k + i);
                match count.get_mut(&(k + i)) {
                    None => {
                        return false;
                    }
                    Some(c) => {
                        if *c == 0 {
                            return false;
                        }
                        *c -= 1;
                        continue;
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
            crate::Solution::is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3)
        );
    }

    #[test]
    pub fn test_solution_1() {
        println!(
            "{}",
            crate::Solution::is_n_straight_hand(vec![1, 1, 2, 2, 3, 3], 3)
        );
    }
}
