mod greedy;

pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // SPACE: O(N)
        let mut min_to_reach = vec![i32::MAX; nums.len()];

        min_to_reach[0] = 0;
        // TIME: O(N)
        for (i, &n) in nums.iter().enumerate() {
            // TIME: O(MAX(nums[i]))
            for j in 1..=n {
                let able_to_reach = i + j as usize;
                if able_to_reach >= nums.len() {
                    break;
                }
                min_to_reach[able_to_reach] =
                    i32::min(min_to_reach[able_to_reach], min_to_reach[i] + 1)
            }
        }
        // => TIME: O(N * MAX(nums[i]))

        min_to_reach[min_to_reach.len() - 1]
    }
}

mod test {
    #[test]
    fn test_jump() {
        let resp = crate::Solution::jump(vec![2, 3, 1, 1, 4]);
        println!("resp {}", resp);
    }
}
