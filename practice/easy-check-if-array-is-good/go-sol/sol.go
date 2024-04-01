package gosol

func isGood(nums []int) bool {
	base := len(nums) - 1
	count := make([]int, base+1)

	for i := 0; i < len(nums); i++ {
		// check if contains >= len(n) - 1
		if nums[i] > base {
			return false
		}

		// only the base can exist twice, others can only exist once
		if c := count[nums[i]]; (nums[i] == base && c >= 2) || (nums[i] != base && c >= 1) {
			return false
		}

		count[nums[i]] += 1
	}

	return true
}
