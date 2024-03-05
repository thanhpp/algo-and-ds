package gosol

func canJumpNaive(nums []int) bool {
	target := len(nums) - 1
	if target == 0 {
		return true
	}

	canJumpTo := make(map[int]bool)
	canJumpTo[0] = true // start

	for i := 0; i < len(nums); i++ {
		if !canJumpTo[0] {
			continue
		}

		for j := 1; j <= nums[i]; j++ {
			canJumpTo[i+j] = true
		}
	}

	return canJumpTo[target]
}
