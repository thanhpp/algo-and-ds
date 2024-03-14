pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let total_gas: i32 = gas.iter().sum(); // O(n)
        let total_cost: i32 = cost.iter().sum(); // O(n)

        if total_cost > total_gas {
            return -1;
        }

        let mut res = 0;
        let mut total = 0;

        for i in 0..gas.len() {
            total += gas[i] - cost[i];
            if total < 0 {
                // can not reach from the old res to i --> Reset
                total = 0;

                // start at the next index: i + 1
                // if we are able to start at i, then gas[i] - cost[i] must >= 0
                // but the total < 0 (with old_total >= 0) => gas[i] - cost[i] < 0
                res += 1;

                // Why do we choose the first index that keeps the rest non-negetive?
                // - because we can not start at any previous index
                // - there is only 1 valid answer
                // - the first index -> the longest path can be taken (i -> end), which can produce the largest sum
            }
        }

        res
    }
}
