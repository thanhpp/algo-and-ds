package gosol

import "math"

/*
Explain: for each element, calculate the minimum product and the maximum product at that point

- Why keep both the max product and the min product?
  - in case of having negative numbers between the given array, the negative values can create the maximum prod
    eg: minProd = -1, maxProd = 1, number = -2
    -> minProd * number = 2, maxProd * number = -2

- When a number is 0, reset the maxProd and the minProd. Otherwise, the prod will always be 0
*/
func maxProduct(nums []int) int {
	var ( // Space = O(1)
		// default values
		res              = math.MinInt
		currMax, currMin = 1, 1
	)

	for _, n := range nums { // Time = O(n)
		if n == 0 {
			currMax, currMin = 1, 1 // reset
			res = max(res, n)
			continue
		}
		tmpMax := max(max(currMax*n, currMin*n), n)
		tmpMin := min(min(currMin*n, currMin*n), n)
		currMax = tmpMax
		currMin = tmpMin
		res = max(res, currMax)
	}

	return res
}

func min(a, b int) int {
	if a < b {
		return a
	}

	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
