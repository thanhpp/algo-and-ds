package gosol

func WordBreakNaive(s string, wordDict []string) bool {
	var dict = make(map[string]bool, len(wordDict))
	for _, s := range wordDict {
		dict[s] = true
	}

	return dfsNaive(dict, s, 0, 0)
}

func dfsNaive(dict map[string]bool, s string, start, end int) bool {
	word := s[start:end]

	if end == len(s) {
		return dict[word]
	}

	if dict[word] {
		return dfsNaive(dict, s, start, end+1) || dfsNaive(dict, s, end, end)
	}

	return dfsNaive(dict, s, start, end+1)
}
