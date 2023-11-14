package gosol

import "math"

func maxProductNaive(nums []int) int {
	var max = math.MinInt

	// Try every possible subarray
	// Space = O(1)
	// Time = O(n^2)

	for i := 0; i < len(nums); i++ { // O(n)
		dfs(nums, i, 1, &max) // O(n) -> try every next element
	}

	return max
}

func dfs(nums []int, id int, prod int, max *int) {
	if id >= len(nums) {
		return
	}

	prod *= nums[id]
	if prod > *max {
		*max = prod
	}

	dfs(nums, id+1, prod, max)
}

/* for loop naive
func maxProduct(nums []int) int {
    var max = math.MinInt

    for i := 0; i < len(nums); i++ {
        prod := 1
        for j := i; j < len(nums); j++ {
            prod *= nums[j]
            if prod > max {
                max = prod
            }
        }
    }

    return max
}
*/
