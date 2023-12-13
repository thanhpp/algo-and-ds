package gosol

// COMPLEXITY: len(nums) * sum(nums) * 2
// each index, can reached to the sum(nums) or -sum(nums)

func findTargetSumWaysCache(nums []int, target int) int {
	var cache = make(map[Cache]int)

	v1 := dfsCache(nums, 0, 1, target, 0, cache)
	v2 := dfsCache(nums, 0, -1, target, 0, cache)

	// fmt.Println(cache)

	return v1 + v2
}

// at each idx, there are serveral value can be reached => with each (idx, value) there can only be a fixed count
// for example
// 1 + 1 + 1 - 1 + 1 = 3 & 1 + 1 + 1 + 1 - 1 = 3 (both has the value == idx + 1)
type Cache struct {
	idx   int
	value int
}

func dfsCache(nums []int, idx int, mul int, target int, count int, cache map[Cache]int) int {
	if idx >= len(nums) {
		return count
	}
	target -= mul * nums[idx]
	if target == 0 && idx == len(nums)-1 {
		return count + 1
	}
	if cachedCount, ok := cache[Cache{idx: idx, value: target}]; ok {
		count += cachedCount
		return count
	}

	res := dfsCache(nums, idx+1, 1, target, count, cache) + dfsCache(nums, idx+1, -1, target, count, cache)

	cache[Cache{idx: idx, value: target}] = res

	return res
}
