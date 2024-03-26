package gosol

// IDEA: all words equal -> count everything then redistribute
func makeEqual(words []string) bool {
	// SPACE: O(sum(len(w in words)))
	charCount := make(map[rune]int)
	totalChar := 0

	// TIME: O(len(words) * max(len(w in words)))
	for _, w := range words {
		for _, c := range w {
			charCount[c] += 1
		}
		totalChar += len(w)
	}

	if totalChar%len(words) != 0 {
		return false
	}

	// TIME: O(sum(len(w in words)))
	for _, v := range charCount {
		if v%len(words) != 0 {
			return false
		}
	}

	return true
}
