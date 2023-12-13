pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; coins.len() + 1]; (amount + 1) as usize];

        // base case, there is only one way use any number of coin to sum up to 0 => not use any coin
        for c_idx in 0..coins.len() {
            dp[0][c_idx] = 1;
        }

        // for each amount
        for amt in 1..=amount {
            // try to use each coin
            for (c_idx, &c) in coins.iter().enumerate() {
                // not use this coin (equal to using the previous coin)
                if c_idx > 0 {
                    dp[amt as usize][c_idx] += dp[amt as usize][c_idx - 1]
                }
                // use this coin
                if amt - c > 0 {
                    dp[amt as usize][c_idx] += dp[(amt - c) as usize][c_idx]
                }
            }
        }

        dp[amount as usize][coins.len() - 1]
    }
}
