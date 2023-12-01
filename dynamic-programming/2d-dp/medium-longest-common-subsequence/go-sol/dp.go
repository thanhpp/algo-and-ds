package gosol

func longestCommonSubsequence(text1 string, text2 string) int {
	// make the dp map
	var rows, cols = len(text2), len(text1)
	dp := make([][]int, cols+1)
	for i := range dp {
		dp[i] = make([]int, rows+1)
	}

	// solve from right -> left, bottom -> top
	for col := len(text1) - 1; col >= 0; col-- {
		for row := len(text2) - 1; row >= 0; row-- {
			if text1[col] == text2[row] {
				dp[col][row] = 1 + dp[col+1][row+1]
				continue
			}
			dp[col][row] = max(dp[col][row+1], dp[col+1][row])
		}
	}

	return dp[0][0]
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
