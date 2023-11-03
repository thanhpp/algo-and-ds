package gosol

/*
How to detect a palindrome
- assume that we start from the middle of the palindrome
- continue to expand to both side and check if 2 characters are equal
*/
func longestPalindrome(s string) string {
	var maxL, maxR = 0, 0 // Space: O(1)

	for i := range s { // O(n)
		// odd palindrome: the middle is a single character
		l, r := i, i
		for (l >= 0 && r < len(s)) && s[l] == s[r] { // O(n/2)
			if maxR-maxL < (r - l + 1) {
				maxL = l
				maxR = r + 1
			}
			l--
			r++
		}

		// even palindrome: the middle are 2 characters
		l, r = i, i+1
		for (l >= 0 && r < len(s)) && s[l] == s[r] { // O(n/2)
			if maxR-maxL < (r - l + 1) {
				maxL = l
				maxR = r + 1
			}
			l--
			r++
		}
	}

	return s[maxL:maxR]
}

/*
Why is this a DP problem
- Don't recheck every letter of each substring
- take the old palindrome -> expand it to both sides
*/
