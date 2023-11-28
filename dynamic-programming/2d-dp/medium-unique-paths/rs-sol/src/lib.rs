pub struct Solution {}

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            dp[i][0] = 1 // first row
        }
        for i in 0..n as usize {
            dp[0][i] = 1 // first col
        }

        for i in 1..n as usize {
            // O(n)
            for j in 1..m as usize {
                // O(m)
                dp[j][i] = dp[j - 1][i] + dp[j][i - 1]
            }
        }

        dp[(m - 1) as usize][(n - 1) as usize]
    }
}
