pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![];
    }

    // stable sort keep the order of elements with same values -> cost more time
    nums.sort_unstable();

    let mut resp: Vec<Vec<i32>> = Vec::new();
    for i in 0..nums.len() {
        let curr = nums[i];

        // skip duplicate values
        if i > 0 && nums[i - 1] == curr {
            continue;
        }

        // kinda 2 sum here
        let mut l = i + 1;
        let mut r = nums.len() - 1;

        while l < r {
            let sum = curr + nums[l] + nums[r];
            if sum == 0 {
                resp.push(vec![curr, nums[l], nums[r]]);
                l += 1; // shift to the next element

                // skip duplicate elements
                while l < r && nums[l - 1] == nums[l] {
                    l += 1;
                }

                // nums[i] + nums[l] + nums[r] = 0
                // with nums[l'] != nums [l]
                // nums[i] + nums[l'] + nums[r] != 0
                // should decrease r
                r -= 1;
                continue;
            }

            if sum < 0 {
                l += 1;
                continue;
            }

            r -= 1;
        }
    }
    resp
}
