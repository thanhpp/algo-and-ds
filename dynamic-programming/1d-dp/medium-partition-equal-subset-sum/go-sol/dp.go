package gosol

/*
	Concept: Save all possible sums. For each step, add the current number to all previous sums
	- need 2 equal subset -> each subset sum = sum/2
	- if sum/2 exist
*/

func canPartition(nums []int) bool {
	// TIME: O(n * sum/2)
	// SPACE: O(sum/2)

	var sum int
	for _, n := range nums {
		sum += n
	}

	if sum%2 != 0 { // can not split if the sum is odd
		return false
	}

	var (
		target      = sum / 2
		possibleSum = make(map[int]bool, target)
	)
	possibleSum[0] = true

	for _, n := range nums { // O(n)
		var sums = make([]int, 0, len(possibleSum))
		for s := range possibleSum {
			sums = append(sums, s)
		}
		for _, s := range sums { // O(sum/2)
			tmp := s + n
			if tmp == target {
				// fmt.Println("s", s, "n", n)
				return true
			}
			if tmp > target {
				continue
			}
			possibleSum[tmp] = true
		}
	}

	// fmt.Println("possibleSum", possibleSum)

	return false
}
