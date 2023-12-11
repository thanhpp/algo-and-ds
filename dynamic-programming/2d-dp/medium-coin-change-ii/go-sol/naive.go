package gosol

func changeNaive(amount int, coins []int) int {
	var count = 0
	dfs(amount, coins, 0, &count)

	return count
}

func dfs(amount int, coins []int, idx int, count *int) {
	if idx >= len(coins) {
		return
	}
	if amount == 0 {
		*count += 1
		return
	}
	if amount < 0 {
		return
	}

	// use current coin
	dfs(amount-coins[idx], coins, idx, count)

	// use the next coin
	dfs(amount, coins, idx+1, count)
}
