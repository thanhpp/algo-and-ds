package gosol

func jump(nums []int) int {
	count := 0
	windowRight := 0
	furthest := 0

	for i, n := range nums {
		furthest = max(furthest, i+n)
		if i > windowRight {
			count += 1
			windowRight = furthest
			if windowRight >= len(nums) {
				return count
			}
		}
	}

	return count
}
