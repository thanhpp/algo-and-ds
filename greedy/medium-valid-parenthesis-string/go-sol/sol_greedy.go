package gosol

func checkValidString(s string) bool {
	// SPACE: O(1)
	openMin, openMax := 0, 0 // -> store the possible range of the open parenthesis

	// TIME: O(n)
	for i := range s {
		switch s[i] {
		case '(':
			openMin += 1
			openMax += 1
		case ')':
			openMin = max(0, openMin-1)
			openMax -= 1
		case '*':
			openMin = max(0, openMin-1)
			openMax += 1
		}
		if openMax < 0 {
			return false
		}
	}

	return openMin == 0
}
