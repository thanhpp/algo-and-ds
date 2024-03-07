pub struct Solution {}

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));

        let mut res = Vec::<Vec<i32>>::new();
        let (mut prev_start, mut prev_end) = (intervals[0][0], intervals[0][1]);

        for cur in intervals.iter().skip(1) {
            if cur[0] > prev_end || cur[1] < prev_start {
                res.push(vec![prev_start, prev_end]);
                prev_start = cur[0];
                prev_end = cur[1];
                continue;
            }

            prev_start = prev_start.min(cur[0]);
            prev_end = prev_end.max(cur[1]);
        }

        res.push(vec![prev_end, prev_start]);

        res
    }
}
