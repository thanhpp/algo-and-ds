pub struct Solution {}

impl Solution {
    // Greedy: always take the optimal sub-solution -> optimal solution
    // Idea: traverse backward, if an index can reach the goal -> set is as the new goal
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // SPACE: O(1)
        let mut goal = nums.len() - 1;

        // TIME: O(n)
        for i in (0..nums.len() - 1).rev() {
            if i + nums[i] as usize >= goal {
                goal = i
            }
        }

        goal == 0
    }
}
