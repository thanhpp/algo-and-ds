package gosol

import (
	"fmt"
)

func Change(amount int, coins []int) int {
	if amount == 0 {
		return 0
	}
	if len(coins) == 0 {
		return 0
	}

	var dp = make([][]int, amount+1)
	for i := range dp {
		dp[i] = make([]int, len(coins))
	}

	// base case: use 1 coin only => 1 way
	for coinIdx := 0; coinIdx < len(coins); coinIdx++ {
		dp[0][coinIdx] = 1
	}

	for coinIdx := 0; coinIdx < len(coins); coinIdx++ {
		// with each amount using coin 0..=coinIdx
		for amt := 1; amt <= amount; amt++ {
			coin := coins[coinIdx]
			// not using this coin
			if coinIdx-1 >= 0 {
				dp[amt][coinIdx] = dp[amt][coinIdx-1]
			}
			// use this coin once more
			if amt-coin >= 0 {
				dp[amt][coinIdx] += dp[amt-coin][coinIdx]
			}
		}
	}

	for i := range dp {
		fmt.Println(dp[i])
	}

	return dp[amount][len(coins)-1]
}
