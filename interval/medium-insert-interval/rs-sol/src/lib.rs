mod sorted_interval;

pub struct Solution {}

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals.clone();

        intervals.push(new_interval);
        // TIME: O(n.log(n))
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        println!("sorted {:?}", intervals);

        // TIME: O(n)
        for i in 1..intervals.len() {
            // should merge with previous
            if intervals[i - 1][1] < intervals[i][0] {
                continue;
            }

            // merge
            intervals[i][0] = intervals[i - 1][0];
            if intervals[i][1] < intervals[i - 1][1] {
                intervals[i][1] = intervals[i - 1][1]
            }

            // mark as merged
            intervals[i - 1][0] = -1
        }

        // TIME: O(n)
        intervals
            .iter()
            .filter(|interval| interval[0] != -1)
            .cloned()
            .collect::<Vec<Vec<i32>>>()
    }
}

mod test {
    use crate::Solution;

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
