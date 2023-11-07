package gosol

func countSubstrings(s string) int {
	count := 0

	/*
		Time: O(n^2)
		Space: O(1)
	*/

	for i := 0; i < len(s); i++ { // O(n)
		// odd length
		l, r := i, i
		for (l >= 0 && r < len(s)) && (s[l] == s[r]) { // O(n)
			count++
			l--
			r++
		}

		// even length
		l, r = i, i+1
		for (l >= 0 && r < len(s)) && (s[l] == s[r]) { // O(n)
			count++
			l--
			r++
		}
	}

	return count
}
