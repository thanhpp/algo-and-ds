package gosol

import "fmt"

func coinChange_Naive(coins []int, amount int) int {
	// set dp
	var dp = make([]int, amount+1)
	for i := 1; i < len(dp); i++ {
		dp[i] = -1
	}

	dfs(coins, amount, 0, 0, 0, dp)

	for i := 0; i < len(dp); i++ {
		if dp[i] != -1 {
			fmt.Println("i:", i, "dp[i]:", dp[i])
		}
	}

	return dp[amount]
}

func dfs(coins []int, amount int, idx int, sum int, count int, dp []int) {
	if idx >= len(coins) {
		return
	}

	if sum > amount {
		return
	}

	if dp[sum] == -1 || dp[sum] > count {
		dp[sum] = count
	}

	// add the count at idx
	dfs(coins, amount, idx, sum+coins[idx], count+1, dp)

	// not add the idx coin but add the next coin
	dfs(coins, amount, idx+1, sum, count, dp)
}
