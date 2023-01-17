fn main() {}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut s = std::collections::HashSet::<i32>::new();
    for i in nums {
        if s.insert(i) {
            continue;
        }
        return true;
    }

    false
}
