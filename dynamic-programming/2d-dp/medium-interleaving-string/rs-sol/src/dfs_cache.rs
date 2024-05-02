use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        // simple length check
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let mut cache = HashMap::<(usize, usize), bool>::new();

        let (s1, s2, s3): (Vec<char>, Vec<char>, Vec<char>) = (
            s1.chars().collect(),
            s2.chars().collect(),
            s3.chars().collect(),
        );

        Self::dfs_cache(&s1, &s2, &s3, 0, 0, &mut cache)
    }

    fn dfs_cache(
        s1: &[char],
        s2: &[char],
        s3: &[char],
        s1_idx: usize,
        s2_idx: usize,
        cache: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(&v) = cache.get(&(s1_idx, s2_idx)) {
            return v;
        }

        if s1_idx + s2_idx == s3.len() {
            return true;
        }

        let res = match (
            s1_idx < s1.len() && s1[s1_idx] == s3[s1_idx + s2_idx],
            s2_idx < s2.len() && s2[s2_idx] == s3[s1_idx + s2_idx],
        ) {
            (true, false) => {
                // can take s1[i], but can not take s2[j]
                Self::dfs_cache(s1, s2, s3, s1_idx + 1, s2_idx, cache)
            }
            (false, true) => {
                // can take s2[j], but can not take s1[i]
                Self::dfs_cache(s1, s2, s3, s1_idx, s2_idx + 1, cache)
            }
            (true, true) => {
                // can take both
                Self::dfs_cache(s1, s2, s3, s1_idx + 1, s2_idx, cache)
                    || Self::dfs_cache(s1, s2, s3, s1_idx, s2_idx + 1, cache)
            }
            (false, false) => false,
        };
        cache.insert((s1_idx, s2_idx), res);

        res
    }
}
