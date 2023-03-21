struct Solution;

// 1 ms, 2.2 MB
impl Solution {
    // NOTE: Rust use usize for slice index, so it is easy to cause an overflow if m = 0 and r = m - 1;
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, (nums.len() - 1));
        let mut min = nums[0];

        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m] < min {
                min = nums[m];
            }

            // we know that the pivot is in the right of the middle index
            // so there must be a smaller value in the right
            if nums[r] < nums[m] {
                l = m + 1;
            } else {
                // this condition is used to protect the application from causing an
                // usize overflow error
                if m == 0 {
                    break;
                }
                r = m - 1;
            }
        }

        min
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        assert_eq!(super::Solution::find_min(vec![3, 4, 5, 1, 2]), 1)
    }
    #[test]
    fn test_2() {
        assert_eq!(super::Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0)
    }
    #[test]
    fn test_3() {
        assert_eq!(super::Solution::find_min(vec![11, 13, 15, 17]), 11)
    }
    #[test]
    fn test_4() {
        assert_eq!(super::Solution::find_min(vec![1]), 1)
    }
    #[test]
    fn test_5() {
        assert_eq!(super::Solution::find_min(vec![2, 1]), 1)
    }
}
