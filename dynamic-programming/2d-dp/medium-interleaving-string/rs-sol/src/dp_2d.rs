pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // simple length check
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let (s1, s2, s3): (Vec<char>, Vec<char>, Vec<char>) = (
            s1.chars().collect(),
            s2.chars().collect(),
            s3.chars().collect(),
        );

        let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];
        // dp[i][j]
        // can use s1[0..i-1] & s[0..j-1] to form s3[0..i + j - 2]
        // i == 0 -> not using s1, j == 0 -> not using s2
        dp[0][0] = true; // use nothing to form nothing

        for i in 0..(s1.len() + 1) {
            for j in 0..(s2.len() + 1) {
                // skip base case
                if i + j == 0 {
                    continue;
                }
                let k = i + j - 1; // s3[k]

                // check match
                dp[i][j] = (i > 0 && s1[i - 1] == s3[k] && dp[i - 1][j]) /*can use s1[i-1] <=> can use s1[i-2] */
                    || (j > 0 && s2[j - 1] == s3[k] && dp[i][j - 1]) /*can use s2[j-1] <=> can use s2[j-2]*/;
            }
        }

        dp[s1.len()][s2.len()]
    }
}
