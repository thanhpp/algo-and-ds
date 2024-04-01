package gosol

func partitionLabels(s string) []int {
	// record last index of each unique character
	// use it to define the end of a partition, update it all the way while iterate

	// Get last index
	lastIdx := make(map[byte]int)
	for i := len(s) - 1; i >= 0; i-- {
		if _, ok := lastIdx[s[i]]; ok {
			continue
		}
		lastIdx[s[i]] = i
	}

	var res []int

	var start = 0
	for start < len(s) {
		end := lastIdx[s[start]]

		for i := start; i <= end; i++ {
			end = max(lastIdx[s[i]], end)
		}
		res = append(res, end-start+1)
		start = end + 1

	}

	return res
}
