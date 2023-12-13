#![allow(dead_code)]

use itertools::Itertools;

// https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd0ohrj/?utm_source=share&utm_medium=web2x&context=3

pub fn solve(spring: &str, counts: impl Iterator<Item = usize>) -> usize {
    let counts = counts.collect_vec();

    // remove unnecessary positions (last ...)
    // add a base case 0 spring -> . -> 0
    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();
    println!("{:?}", spring);

    // dp[i][j] := Number of arrangements of the first j springs into the first i locations
    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in counts {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0; // a continuos group of placements

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            // consider it is . => the same arrangements as previous
            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            // if we have a larger chunk than count, then check if the start of the chunk is a spring
            // if it is a spring => then the length of the chunk
            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
        println!("dp {:?}", dp);
    }

    *dp.last().unwrap()
}

mod test {
    #[test]
    fn test_solve() {
        println!(
            "{}",
            crate::part_2::solve(".??.??.?##.", vec![1_usize, 1, 3].into_iter())
        )
    }
}
