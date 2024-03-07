package gosol

import "slices"

func merge(intervals [][]int) [][]int {
	// sort intervals by start
	// TIME: O(nlogn)
	slices.SortFunc(intervals, func(a, b []int) int {
		if a[0] < b[0] {
			return -1
		}
		if a[0] == b[0] {
			return 0
		}
		return 1
	})

	var (
		// SPACE: O(n)
		res                [][]int
		prevStart, prevEnd = intervals[0][0], intervals[0][1]
	)

	// For each interval, time complexity: O(n)
	for i := 1; i < len(intervals); i++ {
		// check overlap
		if intervals[i][0] > prevEnd || intervals[i][1] < prevStart {
			// no overlap, add to result and update prev
			res = append(res, []int{prevStart, prevEnd})
			prevStart = intervals[i][0]
			prevEnd = intervals[i][1]
			continue
		}

		// overlapped, merge
		prevStart = min(prevStart, intervals[i][0])
		prevEnd = max(prevEnd, intervals[i][1])
	}

	// append last one
	res = append(res, []int{prevStart, prevEnd})

	return res
}
