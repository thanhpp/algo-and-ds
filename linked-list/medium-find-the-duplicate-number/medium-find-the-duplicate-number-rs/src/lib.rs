// 15ms, 3.1MB
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    /*
    because the nums is consists of [1, n] with length n + 1
    --> any value in the nums is a valid index
    Treat the vector as a linked list, which the value of a index is the next index
    use the floyd algorithm like in linked list
    */

    let mut slow = nums[0];
    let mut fast = nums[0];

    /*
    X = distance from the start point to the start of the loop
    Y = distance between the loop start & the point the both slow and fast meet
    C = cycle length
    */

    loop {
        // slow = slow.next
        slow = nums[slow as usize];
        // fast = fast.next.next
        fast = nums[nums[fast as usize] as usize];

        // which mean there are cycles, which make slow and fast meet each other
        // distance(slow) = X + Y + s*C
        // distance(fast) = X + Y + f*C
        // because the fast move twice as fast as the slow
        // distance(fast) = 2*distance(slow)
        // X + Y + f*C = 2*(X + Y + s*C)
        // X + Y + f*C = 2*X + 2*Y + 2*s*C
        // X + Y = 2*s*C - f*C = k*C (k = 2*s - f)
        // X = k*C - Y
        if slow == fast {
            // reset the slow to the first index
            // distance(slow) = 0
            slow = nums[0];
            // then move both slow and fast at the same speed
            // to make slow and fast meet at the start of the cycle
            while slow != fast {
                slow = nums[slow as usize];
                fast = nums[fast as usize];
            }
            // slow == fast
            // assume that slow and fast meet at the start of the cycle
            // distance(slow) = X
            // distance(fast) = X + Y + f*C + X = X + Y + f*C + k*C - Y = X + (f + k)*C
            // => fast has completed some multiple time of the loop

            return slow;
        }
    }
}

// ref: https://www.geeksforgeeks.org/floyds-cycle-finding-algorithm/
