// 2 ms, 2.2 MB
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r): (usize, usize) = (0, nums.len() - 1);
    while l <= r {
        let m = l + (r - l) / 2;
        if nums[m] == target {
            return m as i32;
        }
        // println!("l {} | r {} | m {} | nums[m] {} | nums [l] {} | nums[r] {}", l, r, m, nums[m], nums[l], nums[r]);
        if nums[l] <= nums[m] {
            // no rotate in [l, m]
            if nums[l] <= target && nums[m] > target {
                // target maybe in [l, m]
                if m == 0 {
                    break;
                }
                r = m - 1;
            } else {
                // target is not in [l, m] -> search [m, r]
                l = m + 1;
            }
        } else {
            // rotate in [l, m] -> no rotate in [m, r] (increase order)
            if nums[r] >= target && nums[m] < target {
                // target maybe in [m, r]
                l = m + 1;
            } else {
                if m == 0 {
                    break;
                }
                r = m - 1;
            }
        }
        // println!("m {}", m);
    }
    -1
}
