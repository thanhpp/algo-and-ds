#![allow(dead_code)]

use itertools::Itertools;

// https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd0ohrj/?utm_source=share&utm_medium=web2x&context=3

pub fn solve(spring: &str, counts: &[usize]) -> usize {
    // remove unnecessary positions (last ...)
    // add a base case 0 spring -> . -> 0
    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();
    // println!("{:?}", spring);

    // dp[i][j] := Number of arrangements of the first j springs into the first i locations
    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for &count in counts {
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
        // println!("dp {:?}", dp);
    }

    *dp.last().unwrap()
}

pub fn my_solve(positions: &[char], groups: &[usize]) -> usize {
    // refine the input
    // - add a base case: start with .
    // - remove unnecessary cases (ending .)
    let positions: Vec<char> = format!(
        ".{}",
        positions.iter().collect::<String>().trim_end_matches('.')
    )
    .chars()
    .collect();

    // create a dp table
    // dp[pos][gr] = numbers of possible arrangements of the 0..gr groups into positions 0..pos
    let mut dp = vec![vec![0; positions.len() + 1]; groups.len() + 1];
    // building base cases (possible arragements of 0 group into all positions)
    for (pos_idx, &pos) in positions.iter().enumerate() {
        if pos == '#' {
            // until reached the first spring, there is only 1 way to place 0 group (do nothing)
            break;
        }
        dp[0][pos_idx] = 1
    }

    // build the dp table
    for (group_index, &group_length) in groups.iter().enumerate() {
        let mut chunk_length = 0; // length of the continous postions which are not ground (not .)

        for (pos_idx, &pos) in positions.iter().enumerate() {
            // calculate the chunk length
            if pos != '.' {
                chunk_length += 1
            } else {
                // reset
                chunk_length = 0;
            }

            // the position is . or considering ? as .
            if pos != '#' {
                // equal to the number of arrangements of the previous position
                dp[group_index + 1][pos_idx + 1] += dp[group_index + 1][pos_idx];
            }

            // the chunk is enough to fit the group
            // assume the current group is put at the end (the position before the group must not be a spring, if there is a spring -> invalid length)
            // equal to the number of the arrangements of the previous group (add this group only maintain the number of arrangement from 0..group_i -> 0..group_i+1)
            if chunk_length >= group_length && positions[pos_idx - group_length] != '#' {
                dp[group_index + 1][pos_idx + 1] += dp[group_index][pos_idx - group_length]
            }
        }
    }

    // for r in dp.iter() {
    //     println!("{:?}", r);
    // }

    dp[groups.len()][positions.len()]
}

mod test {
    #[test]
    fn test_solve() {
        println!("{}", crate::part_2::solve(".??.??.?##.", &[1_usize, 1, 3]))
    }
}
