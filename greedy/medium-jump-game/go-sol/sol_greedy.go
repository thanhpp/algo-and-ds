package gosol

func canJump(nums []int) bool {
	goal := len(nums) - 1

	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i]+i >= goal {
			goal = i
		}
	}

	return goal == 0
}
