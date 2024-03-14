mod greedy;

pub struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        for i in 0..gas.len() {
            // TIME: O(n)
            let mut current_stop = i;
            let mut total_gas = gas[i];
            for _ in 0..gas.len() {
                // TIME: O(n)
                if total_gas < cost[current_stop] {
                    break;
                }
                total_gas -= cost[current_stop];
                current_stop = (current_stop + 1) % gas.len();
                total_gas += gas[current_stop];
                if current_stop == i {
                    return (i) as i32;
                }
            }
        }
        // -> TIME: O(n^2)
        // SPACE: O(1)

        -1
    }
}

mod test {
    #[test]
    fn test_1() {
        let sol = crate::Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
        println!("test_1: {}", sol);
    }
}
