package gosol

import "slices"

func thirdMaxSort(nums []int) int {
	slices.Sort(nums) // TIME: O(nlogn)

	// SPACE: O(1)
	maxValue := nums[len(nums)-1]
	lastValue := maxValue
	uniqueCount := 1

	// TIME: O(n)
	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i] != lastValue {
			lastValue = nums[i]
			uniqueCount++
		}
		if uniqueCount == 3 {
			return lastValue
		}
	}

	return maxValue
}
