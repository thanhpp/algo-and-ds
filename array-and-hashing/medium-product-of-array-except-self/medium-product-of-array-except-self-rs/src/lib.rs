// this use O(1) space but still use division - handle lots of edge cases
// 8 ms - 3.3 MB
pub fn product_except_self_v1(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }

    let mut prod: i32 = 1;
    let mut zero_count: i32 = 0;
    for i in nums.iter() {
        if *i == 0 {
            zero_count += 1;
            continue;
        }
        prod *= *i;
    }

    if zero_count as usize == nums.len() {
        return vec![0; nums.len()];
    }

    let mut resp: Vec<i32> = Vec::new();
    for i in nums.iter() {
        if zero_count == 1 && *i == 0 {
            resp.push(prod);
            continue;
        }
        if zero_count != 0 {
            resp.push(0);
            continue;
        }
        resp.push(prod / *i);
    }

    resp
}

// using 2 vector as postfix & prefix products
// resp[i] = prefix[i - 1] * postfix[i + 1]
// 13 ms - 3.2 MB
pub fn product_except_self_v2(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() == 0 {
        return vec![];
    }
    let length = nums.len();
    let mut prefix: Vec<i32> = vec![1; nums.len()];
    let mut postfix: Vec<i32> = vec![1; nums.len()];
    // build prefix
    prefix[0] = nums[0];
    for i in 1..length {
        prefix[i] = prefix[i - 1] * nums[i];
    }
    // build postfix
    postfix[length - 1] = nums[length - 1];
    for i in (0..length - 1).rev() {
        // create a reserve iterator
        postfix[i] = postfix[i + 1] * nums[i];
    }

    let mut resp: Vec<i32> = Vec::new();
    for i in 0..length {
        if i == 0 {
            resp.push(postfix[i + 1]);
            continue;
        }
        if i == length - 1 {
            resp.push(prefix[i - 1]);
            continue;
        }
        resp.push(prefix[i - 1] * postfix[i + 1]);
    }

    resp
}

// O(1) space, don't include the return vector
// 12 ms - 3.2 MB
pub fn product_except_self_v3(nums: Vec<i32>) -> Vec<i32> {
    let length = nums.len();
    if length == 0 {
        return vec![];
    }

    // I will use the resp as the storage for both prefix and postfix vector
    let mut resp: Vec<i32> = vec![1; nums.len()];
    // resp[i] = prefix of nums[i]
    for i in 0..length - 1 {
        resp[i + 1] = resp[i] * nums[i];
    }

    // resp[i] *= postfix of nums[i]
    // skip the last element because it has no postfix
    let mut postfix = 1;
    for i in (0..length - 1).rev() {
        // create a reserve iterator
        postfix *= nums[i + 1]; // postfix product of nums [i]
        resp[i] *= postfix;
    }

    resp
}
