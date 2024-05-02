pub struct Solution {}

// TIME LIMIT EXCEEDED
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

        Self::dfs(&s1, &s2, &s3, 0, 0)
    }

    fn dfs(s1: &[char], s2: &[char], s3: &[char], i: usize, j: usize) -> bool {
        if i + j == s3.len() {
            return true;
        }

        match (
            i < s1.len() && s1[i] == s3[i + j],
            j < s2.len() && s2[j] == s3[i + j],
        ) {
            (true, false) => {
                // can take s1[i], but can not take s2[j]
                Self::dfs(s1, s2, s3, i + 1, j)
            }
            (false, true) => {
                // can take s2[j], but can not take s1[i]
                Self::dfs(s1, s2, s3, i, j + 1)
            }
            (true, true) => {
                // can take both
                Self::dfs(s1, s2, s3, i + 1, j) || Self::dfs(s1, s2, s3, i, j + 1)
            }
            (false, false) => false,
        }
    }
}
