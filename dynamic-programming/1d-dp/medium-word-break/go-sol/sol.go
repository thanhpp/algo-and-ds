package gosol

/*
Concept:
  - breaks s into words by trying each word in the word dict
  - starts from the end of s
  - at the index i, record if it is possible to break from i to the end of s
*/
func wordBreak(s string, wordDict []string) bool {
	/*
		Space: O(n)
		Time: O(len(s) * len(dict))
	*/

	var canBreakFromIdx = make([]bool, len(s)+1) // Space: O(dict)
	canBreakFromIdx[len(s)] = true               // there is nothing

	for i := len(s) - 1; i >= 0; i-- { // Time: O(len(s))
		for _, word := range wordDict { // Time: O(dict)
			isInBound := i+len(word) <= len(s)
			if !isInBound {
				continue
			}
			isEqual := s[i:i+len(word)] == word
			// can continue to break using this word
			if isEqual && canBreakFromIdx[i+len(word)] {
				canBreakFromIdx[i] = true
				break
			}
		}
	}

	return canBreakFromIdx[0]
}
