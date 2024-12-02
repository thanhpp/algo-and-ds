pub struct Solution {}

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::<usize>::from_iter(0..=nums.len());

        for n in nums {
            set.remove(&(n as usize));
        }

        *set.iter().next().unwrap() as i32
    }
}
