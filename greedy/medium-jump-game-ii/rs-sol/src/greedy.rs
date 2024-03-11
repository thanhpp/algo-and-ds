pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // SPACE: O(1)
        let mut step = 0;
        let (mut level_left, mut level_right) = (0, 0);

        // until the level exceed the goal -> can reach the goal from last step
        while level_right < nums.len() - 1 {
            let mut farthest = 0; // will determine the next level right

            // try to jump at every index of the level
            for i in level_left..=level_right {
                farthest = farthest.max(nums[i] as usize + i) // compare with farthest can jump at index i
            }

            // update the next window
            level_left += 1;
            level_right = farthest;
            step += 1;
        }
        // => TIME: O(n + max(nums)) -> BFS complexity = O(V + E) ~ V = len(nums) & E = max(nums[i])

        step
    }
}
