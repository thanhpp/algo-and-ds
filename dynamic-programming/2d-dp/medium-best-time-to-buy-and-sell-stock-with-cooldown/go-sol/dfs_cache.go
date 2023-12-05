package gosol

import "fmt"

func maxProfit(prices []int) int {
	// TIME: O(n): build max profit each day backwards
	// SPACE: O(2n): 2 decisions each day

	var cache = make(map[string]int)

	dfsWithCache(prices, 0, Buy, cache)

	return cache[cacheKey(0, Buy)] // need to buy before sell
}

type Action bool

const (
	Buy  Action = true
	Sell Action = false
)

func dfsWithCache(prices []int, day int, action Action, cache map[string]int) int {
	/*
		CONCEPT: Calculate the max profit of each day by action (buy, sell)
		- If buy: profit -= price[day]
		- If sell: proft += price[day] (also need to cooldown for 1 day)

		profit of day i
		- Buy at day i = profit sell at the next day (i + 1) - today price
		- Sell at day i = profit buy at the day after tomorrow + today price
	*/

	if day >= len(prices) {
		return 0
	}

	key := cacheKey(day, action)
	if maxProfit, ok := cache[key]; ok {
		return maxProfit
	}

	switch action {
	case Buy:
		buyProfit := dfsWithCache(prices, day+1, Sell, cache) - prices[day]
		cooldown := dfsWithCache(prices, day+1, Buy, cache)
		cache[key] = max(buyProfit, cooldown)

	case Sell:
		sellProfit := dfsWithCache(prices, day+2, Buy, cache) + prices[day] // 1 day cooldown
		cooldown := dfsWithCache(prices, day+1, Sell, cache)
		cache[key] = max(sellProfit, cooldown)
	}

	return cache[key]
}

func cacheKey(day int, action Action) string {
	if action == Buy {
		return fmt.Sprintf("%d-BUY", day)
	}

	return fmt.Sprintf("%d-SELL", day)
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
