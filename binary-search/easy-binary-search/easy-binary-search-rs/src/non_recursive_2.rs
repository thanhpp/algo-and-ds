// 5ms - 2,2 MB
pub fn search(nums: &Vec<i32>, target: i32) -> i32 {
    let (l, r): (i32, i32) = (0, nums.len());

    while l <= r {
        // if l, r, m is usize, in case l == r == 1
        // => m = 0
        // => r = -1 --> BREAK
        let m = l + (r - l) / 2; // if we use (r + l) / 2, then we can cause an integer overflow
        if nums[m] == target {
            return m;
        }

        if nums[m] < target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    -1
}
