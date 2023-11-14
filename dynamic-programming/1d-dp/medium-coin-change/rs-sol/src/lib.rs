pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount_usize = amount as usize;
        let mut dp = vec![-1; amount_usize + 1];

        dp[amount_usize] = 0;

        for i in (0..=amount_usize).rev() {
            if dp[i] != -1 {
                let i_i32 = i as i32;
                for c in coins.iter() {
                    if *c > i_i32 {
                        continue;
                    }
                    let count = dp[i] + 1;
                    let idx = (i_i32 - *c) as usize;
                    if dp[idx] == -1 || dp[idx] > count {
                        dp[idx] = count
                    }
                }
            }
        }

        dp[0]
    }
}
