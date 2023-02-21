use std::collections::HashSet;

// 28ms, 4MB
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut hs = HashSet::<i32>::new();

    for i in nums {
        hs.insert(i);
    }

    // find head - no left value
    let mut max = 0;
    for i in hs.iter() {
        let left_val = i - 1;
        if hs.contains(&left_val) {
            // is a head
            let mut length = 1;
            loop {
                let next = *i + length;
                if !hs.contains(&next) {
                    break;
                }
                length += 1;
            }
            if max < length {
                max = length;
            }
        }
    }

    max
}
