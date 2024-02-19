pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // handle edge case
        if intervals.is_empty() {
            return vec![new_interval];
        }

        let mut res = Vec::<Vec<i32>>::new();
        let mut i = 0;
        let mut added = false;
        loop {
            if i >= intervals.len() {
                break;
            }

            let next_interval: Vec<i32>;
            if !added && new_interval[0] < intervals[i][0] {
                // add the smaller
                added = true;
                next_interval = new_interval.clone();
            } else {
                next_interval = intervals[i].clone();
                i += 1;
            }

            // add or merge
            if res.is_empty() || res.last().unwrap()[1] < next_interval[0] {
                res.push(next_interval);
                continue;
            }

            // merge
            let length = res.len();
            if res[length - 1][0] > next_interval[0] {
                res[length - 1][0] = next_interval[0];
            }

            if res[length - 1][1] < next_interval[1] {
                res[length - 1][1] = next_interval[1];
            }
        }

        // add to last
        if !added {
            // add or merge
            if res.is_empty() || res.last().unwrap()[1] < new_interval[0] {
                res.push(new_interval);
            } else {
                // merge
                let length = res.len();
                if res[length - 1][0] > new_interval[0] {
                    res[length - 1][0] = new_interval[0];
                }

                if res[length - 1][1] < new_interval[1] {
                    res[length - 1][1] = new_interval[1];
                }
            }
        }

        res
    }
}

mod test {
    use crate::sorted_interval::Solution;

    #[test]
    fn test_insert_1() {
        println!(
            "{:?}",
            Solution::insert([[1, 3].to_vec(), [6, 9].to_vec()].to_vec(), [2, 5].to_vec())
        )
    }

    #[test]
    fn test_insert_2() {
        println!(
            "{:?}",
            Solution::insert(
                [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]]
                    .map(|s| s.to_vec())
                    .to_vec(),
                [4, 8].to_vec()
            )
        )
    }
}
