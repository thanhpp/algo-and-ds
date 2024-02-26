pub struct Solution {}
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals.clone();

        // O(nlogn)
        // NOTE: unstable sort reduce the run time by 42% (80 -> 56)
        intervals.sort_unstable_by(|a, b| match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            _ => a[0].cmp(&b[0]),
        });

        let mut removed = 0;
        let mut last_non_overlap_interval_end = intervals[0][1];

        // O(n)
        for curr in intervals.iter().skip(1) {
            // check if overlap with the previous
            if curr[0] >= last_non_overlap_interval_end {
                // no overlap
                last_non_overlap_interval_end = curr[1];
                continue;
            }

            // overlap, remove the one that has the greater end
            removed += 1;
            if curr[1] > last_non_overlap_interval_end {
                continue;
            }
            last_non_overlap_interval_end = curr[1]
        }

        removed
    }
}
