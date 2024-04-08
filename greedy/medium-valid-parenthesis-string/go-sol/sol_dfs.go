package gosol

// BRUTE FORCE - DFS:
// Try all possible outcomes, * can be (, ) or ''
// Add a cache to store the result of each decision branch
// Without cache: time = 3^n, space = 1
// With cache: time = n^3 (visit each edge only once), space = 3n (record every edges)

func checkValidStringDFS(s string) bool {
	cache := make(map[[2]int]bool) // [idx - open] -> result

	return dfs([]byte(s), 0, 0, cache)
}

func dfs(s []byte, idx int, open int, cache map[[2]int]bool) bool {
	if open < 0 || (idx == len(s) && open != 0) {
		return false
	}
	if open == 0 && idx == len(s) {
		return true
	}
	if v, ok := cache[[2]int{idx, open}]; ok {
		return v
	}

	var isValid = false
	switch s[idx] {
	case '(':
		isValid = dfs(s, idx+1, open+1, cache)
	case ')':
		isValid = dfs(s, idx+1, open-1, cache)
	case '*':
		// TIME COMPLEXITY: 3^n
		isValid = dfs(s, idx+1, open+1, cache) || dfs(s, idx+1, open-1, cache) || dfs(s, idx+1, open, cache)
	}
	cache[[2]int{idx, open}] = isValid

	return isValid
}
