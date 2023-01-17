use std::vec;

fn main() {
    println!("Hello, world!");
}

// 2 ms - 2.6 MB
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = std::collections::HashMap::<i32, usize>::new();

    for i in 0..nums.len() {
        let need = target - nums[i];
        match hm.get(&need) {
            Some(idx) => {
                return vec![i as i32, *idx as i32];
            }
            None => hm.insert(nums[i], i),
        };
    }

    vec![]
}
