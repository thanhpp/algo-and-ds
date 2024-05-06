pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        // easy & edge cases
        if word1.is_empty() || word2.is_empty() {
            return word1.len().abs_diff(word2.len()) as i32;
        }

        let (word1, word2): (Vec<char>, Vec<char>) =
            (word1.chars().collect(), word2.chars().collect());

        // dp[i][j] = min_distance(word1[0..i], word2[0..j])
        let mut dp = vec![vec![i32::MAX; word2.len() + 1]; word1.len() + 1];

        // base case:
        {
            // mathching nothing
            dp[word1.len()][word2.len()] = 0;
            // base case: empty into sth => insert all
            for i in 0..dp.len() {
                dp[i][word2.len()] = (word1.len() - i) as i32;
            }
            for j in 0..=word2.len() {
                dp[word1.len()][j] = (word2.len() - j) as i32;
            }
        }

        for i in (0..word1.len()).rev() {
            for j in (0..word2.len()).rev() {
                if word1[i] == word2[j] {
                    dp[i][j] = dp[i + 1][j + 1];
                    continue;
                }

                dp[i][j] = /*insertion*/dp[i + 1][j].min(/*deletion*/dp[i][j + 1]).min(/*replace*/dp[i + 1][j + 1]) + 1;
            }
        }

        // for i in dp.iter() {
        //     println!("{:?}", i);
        // }

        dp[0][0]
    }
}

mod test {
    #[test]
    fn test_1() {
        println!(
            "{}",
            crate::dp_2d::Solution::min_distance(String::from("horse"), String::from("ros"))
        )
    }

    #[test]
    fn test_2() {
        println!(
            "{}",
            crate::dp_2d::Solution::min_distance(
                String::from("intention"),
                String::from("execution")
            )
        )
    }

    #[test]
    fn test_3() {
        println!(
            "{}",
            crate::dp_2d::Solution::min_distance(String::from(""), String::from("a"))
        )
    }

    #[test]
    fn test_4() {
        println!(
            "{}",
            crate::dp_2d::Solution::min_distance(
                String::from("zoologicoarchaeologist"),
                String::from("zoogeologist")
            )
        )
    }
}
