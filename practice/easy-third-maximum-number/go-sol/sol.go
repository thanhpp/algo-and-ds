package gosol

import (
	"fmt"
	"math"
)

// TIME: O(3n) ~ O(n)
func thirdMax(nums []int) int {
	var (
		maxVal       = math.MinInt64
		secondMaxVal = math.MinInt64
		thirdMaxVal  = math.MinInt64
	)

	for i := 0; i < len(nums); i++ {
		v := nums[i]
		if v > maxVal {
			thirdMaxVal = secondMaxVal
			secondMaxVal = maxVal
			maxVal = v
			fmt.Println("1 - ", maxVal, secondMaxVal, thirdMaxVal)
			continue
		}
		if v == maxVal {
			continue
		}
		if v > secondMaxVal {
			thirdMaxVal = secondMaxVal
			secondMaxVal = maxVal
			fmt.Println("2 - ", maxVal, secondMaxVal, thirdMaxVal)
			continue
		}
		if v == secondMaxVal {
			continue
		}
		if v > thirdMaxVal {
			thirdMaxVal = v
			fmt.Println("3 - ", maxVal, secondMaxVal, thirdMaxVal)
			continue
		}
	}

	if thirdMaxVal == math.MinInt { // not found third max value
		return maxVal
	}

	return thirdMaxVal
}
