pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    /*
    Time Limit Exceeded
    1145 / 1146 testcases passed
    */

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2): (Vec<char>, Vec<char>) =
            (word1.chars().collect(), word2.chars().collect());

        let mut cache: HashMap<(usize, usize, i32), i32> = HashMap::new();

        Self::dfs(&word1, &word2, 0, 0, 0, &mut cache)
    }

    fn dfs(
        word1: &[char],
        word2: &[char],
        i1: usize,
        i2: usize,
        cost: i32,
        cache: &mut HashMap<(usize, usize, i32), i32>,
    ) -> i32 {
        let key = (i1, i2, cost);
        if let Some(&c) = cache.get(&key) {
            return c;
        }

        if i2 == word2.len() {
            // delete
            let new_cost = cost + (word1.len() - i1) as i32;
            cache.insert(key, new_cost);
            return new_cost;
        }
        if i1 == word1.len() {
            // add
            let new_cost = cost + (word2.len() - i2) as i32;
            cache.insert(key, new_cost);
            return new_cost;
        }

        let (insert, remove) = (
            Self::dfs(word1, word2, i1, i2 + 1, cost + 1, cache),
            Self::dfs(word1, word2, i1 + 1, i2, cost + 1, cache),
        );
        if word1[i1] == word2[i2] {
            let skip = Self::dfs(word1, word2, i1 + 1, i2 + 1, cost, cache);
            let new_cost = insert.min(remove).min(skip);
            cache.insert(key, new_cost);
            return new_cost;
        }

        let replace = Self::dfs(word1, word2, i1 + 1, i2 + 1, cost + 1, cache);

        let new_cost = insert.min(remove).min(replace);
        cache.insert(key, new_cost);
        new_cost
    }
}

mod test {
    #[test]
    fn test_1() {
        println!(
            "{}",
            crate::dfs_cache::Solution::min_distance(String::from("horse"), String::from("ros"))
        )
    }

    #[test]
    fn test_2() {
        println!(
            "{}",
            crate::dfs_cache::Solution::min_distance(
                String::from("intention"),
                String::from("execution")
            )
        )
    }
}
