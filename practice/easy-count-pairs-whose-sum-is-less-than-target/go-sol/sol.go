package gosol

func countPairs(nums []int, target int) int {
	count := 0

	for i := 0; i < len(nums); i++ {
		for j := i + 1; j < len(nums); j++ {
			s := nums[i] + nums[j]
			if s < target {
				count += 1
			}
		}
	}

	return count
}
