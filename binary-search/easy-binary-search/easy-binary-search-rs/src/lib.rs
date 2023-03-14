mod non_recursive;

// 2ms; 2.2MB
pub fn search(nums: &Vec<i32>, target: i32) -> i32 {
    return bin_search(nums, target, 0, nums.len());
}

pub fn bin_search(nums: &Vec<i32>, target: i32, l: usize, r: usize) -> i32 {
    println!("l: {}, r: {}", l, r);

    let mid_idx = (l + r) / 2;
    if nums[mid_idx] == target {
        return mid_idx as i32;
    }
    if r - l <= 1 {
        return -1;
    }

    if nums[mid_idx] < target {
        return bin_search(nums, target, mid_idx, r);
    }
    return bin_search(nums, target, l, mid_idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_search() {
        // assert_eq!(search(&vec![-1, 0, 3, 5, 9, 12], 9), 4);
        // assert_eq!(search(&vec![-1, 0, 3, 5, 9, 12], 2), -1);
        // assert_eq!(search(&vec![2, 5], 5), 1);
        println!("test 6");
        assert_eq!(search(&vec![2, 5], 2), 0);
    }
}
