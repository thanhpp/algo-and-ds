package gosol

func insert(intervals [][]int, newInterval []int) [][]int {
	// newInterval is in between the intervals
	var (
		res [][]int
	)

	for i := 0; i < len(intervals); i++ {
		if newInterval[1] < intervals[i][0] {
			// not overlap with the current interval
			// also not overlap with the response
			res = append(res, newInterval)       // add the new interval
			return append(res, intervals[i:]...) // add the rest
		}

		if newInterval[0] > intervals[i][1] {
			// the new interval is behind & not overlap with the current interval
			res = append(res, intervals[i])
			continue
		}

		// has overlap, need to merge
		// store the merged interval into the new interval
		newInterval[0] = min(newInterval[0], intervals[i][0])
		newInterval[1] = max(newInterval[1], intervals[i][1])
	}

	// add the new interval because the is it not returned above (before one of the interval in the intervals list)
	res = append(res, newInterval)

	return res
}
