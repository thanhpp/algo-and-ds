pub struct Solution {}

impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let (mut found_0, mut found_1, mut found_2) = (false, false, false);

        for t in triplets {
            if t[0] > target[0] || t[1] > target[1] || t[2] > target[2] {
                continue;
            }

            found_0 |= t[0] == target[0];
            found_1 |= t[1] == target[1];
            found_2 |= t[2] == target[2];

            if found_0 && found_1 && found_2 {
                return true;
            }
        }

        false
    }
}
