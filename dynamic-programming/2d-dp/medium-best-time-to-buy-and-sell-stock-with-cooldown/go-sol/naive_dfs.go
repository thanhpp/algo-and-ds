package gosol

func maxProfitNaive(prices []int) int {
	var profit = -1

	dfs(prices, 0, -1, false, 0, &profit)

	return profit
}

func dfs(prices []int, day int, buyPrice int, isCooldown bool, profit int, maxProfit *int) {
	// TIME: O(2^n) => each price 2 decisions (buy, cooldown) || (sell, cooldown)
	// SPACE: O(1)

	if profit > *maxProfit {
		*maxProfit = profit
	}
	if day >= len(prices) {
		return
	}

	// do nothing and move to the next day
	if isCooldown {
		dfs(prices, day+1, -1, false, profit, maxProfit)
		return
	}

	// hasn't bought anything
	if buyPrice == -1 {
		// buy today
		dfs(prices, day+1, prices[day], false, profit, maxProfit)

		// not buy today
		dfs(prices, day+1, -1, false, profit, maxProfit)
		return
	}

	// already bought, try to sell
	todayProfit := prices[day] - buyPrice
	if todayProfit >= 0 { // ok to sell, set cooldown
		dfs(prices, day+1, -1, true, profit+todayProfit, maxProfit)
	}

	// not sell to day
	dfs(prices, day+1, buyPrice, false, profit, maxProfit)
}
