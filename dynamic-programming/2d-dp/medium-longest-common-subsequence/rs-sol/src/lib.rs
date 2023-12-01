pub struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (cols, rows) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; rows + 1]; cols + 1];

        let (text1, text2): (Vec<char>, Vec<char>) =
            (text1.chars().collect(), text2.chars().collect());

        for r in (0..rows).rev() {
            for c in (0..cols).rev() {
                if text1[c] == text2[r] {
                    dp[c][r] = 1 + dp[c + 1][r + 1];
                    continue;
                }
                dp[c][r] = dp[c][r + 1].max(dp[c + 1][r]);
            }
        }

        dp[0][0]
    }
}
