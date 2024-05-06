pub struct Solution {}

impl Solution {
    // TIME COMPLEXITY: O(3 ^ max(word1, word2))

    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2): (Vec<char>, Vec<char>) =
            (word1.chars().collect(), word2.chars().collect());

        Self::dfs(&word1, &word2, 0, 0, 0)
    }

    fn dfs(word1: &[char], word2: &[char], i1: usize, i2: usize, cost: i32) -> i32 {
        if i2 == word2.len() {
            // delete
            return cost + (word1.len() - i1) as i32;
        }
        if i1 == word1.len() {
            // add
            return cost + (word2.len() - i2) as i32;
        }

        let (insert, remove) = (
            Self::dfs(word1, word2, i1, i2 + 1, cost + 1),
            Self::dfs(word1, word2, i1 + 1, i2, cost + 1),
        );
        if word1[i1] == word2[i2] {
            let skip = Self::dfs(word1, word2, i1 + 1, i2 + 1, cost);
            return insert.min(remove).min(skip);
        }

        let replace = Self::dfs(word1, word2, i1 + 1, i2 + 1, cost + 1);

        insert.min(remove).min(replace)
    }
}

mod test {
    #[test]
    fn test_1() {
        println!(
            "{}",
            crate::dfs::Solution::min_distance(String::from("horse"), String::from("ros"))
        )
    }

    #[test]
    fn test_2() {
        println!(
            "{}",
            crate::dfs::Solution::min_distance(
                String::from("intention"),
                String::from("execution")
            )
        )
    }
}
