package gosol

import "slices"

func eraseOverlapIntervals(intervals [][]int) int {
	slices.SortFunc[[][]int](
		intervals,
		func(a, b []int) int {
			if a[0] < b[0] {
				return -1
			}
			if a[0] == b[0] {
				return 0
			}
			return 1
		},
	)

	var (
		removeCount        int
		prevStart, prevEnd = intervals[0][0], intervals[0][1]
	)

	for i := 1; i < len(intervals); i++ {
		curr := intervals[i]
		// check if overlap
		if prevStart > curr[1] || prevEnd < curr[0] {
			// no overlap
			prevStart = curr[0]
			prevEnd = curr[1]
			continue
		}

		// has overlap, keep the one with the smaller end
		removeCount += 1
		if prevEnd < curr[1] {
			// keep previous
			continue
		} else {
			// keep current
			prevStart = curr[0]
			prevEnd = curr[1]
		}
	}

	return removeCount
}
