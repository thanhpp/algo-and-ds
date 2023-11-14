package gosol

func coinChange(coins []int, amount int) int {
	// set dp
	var dp = make([]int, amount+1)
	for i := 1; i < len(dp); i++ { // 0 always need 0 coint
		dp[i] = -1
	}

	/*
		 dp[i]
			- = -1 (not possible to sum up to this value)
			- = 1 (if i == coin value)
			- = min(dp[i - coin]) cond:  i - coin >= 0 && dp[i-c] != -1
	*/

	for i := 1; i < len(dp); i++ {
		for _, c := range coins {
			if i == c {
				dp[i] = 1
				break
			}
			if i-c >= 0 {
				if dp[i-c] == -1 { // can not sum
					continue
				}
				if dp[i] == -1 || dp[i] > dp[i-c]+1 { // a possible way
					dp[i] = dp[i-c] + 1
				}
			}
		}
	}

	return dp[amount]
}
