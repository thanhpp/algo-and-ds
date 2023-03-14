use std::{
    thread::{self},
    time::Duration,
};

// 2ms; 2.2MB
pub fn search2(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l: usize = 0;
    // use nums.len(), instead of nums.len() - 1
    // mid ~ roundDown((len - 0)/2)
    // if len is even -> no problem
    // if len is odd -> m = len/2 - 0.5 -> m can never equal len -> can not account nums[len]
    let mut r = nums.len();

    loop {
        thread::sleep(Duration::from_secs_f64(0.5));
        let m: usize = (l + r) / 2;
        println!("l: {}, r: {}, m: {}", l, r, m);
        if nums[m] == target {
            return m as i32;
        }

        // r - l = number of remaining elements
        // that means there's only 1 element left
        // -> don't have any other to compare
        if r - l <= 1 {
            return -1;
        }

        if nums[m] < target {
            l = m;
        } else {
            r = m;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_search() {
        // println!("test 1");
        // assert_eq!(search2(&vec![-1, 0, 3, 5, 9, 12], 9), 4);
        // println!("test 2");
        // assert_eq!(search2(&vec![-1, 0, 3, 5, 9, 12], 2), -1);
        // println!("test 3");
        // assert_eq!(search2(&vec![5], -5), -1);
        // println!("test 4");
        // assert_eq!(search2(&vec![5], 5), 0);
        // println!("test 5");
        // assert_eq!(search2(&vec![2, 5], 5), 1);
        println!("test 6");
        assert_eq!(search2(&vec![2, 5], 2), 0);
    }
}
