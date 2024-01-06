package gosol

import "fmt"

func minDistance(word1 string, word2 string) int {
	// dp[i][j] = minDistance(word1[i:], word2[j:])
	var dp = make([][]int, len(word1)+1)
	for i := range dp {
		dp[i] = make([]int, len(word2)+1)
	}
	// set up base cases
	// both empty -> 0 action
	// 1 empty, 1 not -> len(notEmptyWord) (all insert)
	dp[len(word1)][len(word2)] = 0
	for i := 0; i < len(word1); i++ {
		dp[i][len(word2)] = len(word1) - i
	}
	for j := 0; j < len(word2); j++ {
		dp[len(word1)][j] = len(word2) - j
	}

	for i := range dp {
		fmt.Println(dp[i])
	}

	fmt.Println("---")

	// going bottom up
	for i := len(word1) - 1; i >= 0; i-- {
		for j := len(word2) - 1; j >= 0; j-- {
			if word1[i] == word2[j] {
				dp[i][j] = dp[i+1][j+1] // do nothing
			} else {
				dp[i][j] = min(dp[i+1][j+1] /*replace*/, dp[i][j+1] /*delete*/, dp[i+1][j] /*insert*/) + 1
			}
		}
	}

	for i := range dp {
		fmt.Println(dp[i])
	}

	return dp[0][0]
}
