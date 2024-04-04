pub struct Solution {}

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_index = std::collections::HashMap::<char, usize>::new();
        let chars = s.chars().collect::<Vec<char>>();

        for (i, c) in chars.iter().enumerate() {
            match last_index.get_mut(c) {
                None => {
                    last_index.insert(*c, i);
                }
                Some(idx) => *idx = i,
            }
        }

        let mut res = Vec::<i32>::new();

        let (mut start, mut end) = (0, 0);
        let mut curr = 0;
        while start < chars.len() {
            end = end.max(*last_index.get(&chars[curr]).unwrap());
            if curr >= end {
                res.push((end - start + 1) as i32);
                start = end + 1;
            }
            curr += 1;
        }

        res
    }
}
