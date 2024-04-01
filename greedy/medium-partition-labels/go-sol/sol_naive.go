package gosol

func partitionLabelsNaive(s string) []int {
	// count every character - TIME: O(n)
	count := make(map[byte]int)
	for i := range s {
		count[s[i]] += 1
	}

	var res []int

	start := 0
	// check each character once -> O(n)
	for start < len(s) {
		end := start
		exist := make(map[byte]bool) // SPACE: O(n)
		for ; end < len(s); end++ {
			exist[s[end]] = true
			count[s[end]] -= 1

			// check if all of the group count equals to 0
			var isAllZero = true
			// With each character: check all exist -> TIME: O(n)
			for k := range exist {
				if count[k] != 0 {
					isAllZero = false
					break
				}
			}

			if isAllZero {
				res = append(res, end-start+1)
				break
			}
		} // => TIME: O(n^2)
		start = end + 1
	}

	return res
}
