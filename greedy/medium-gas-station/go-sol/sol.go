package gosol

func canCompleteCircuit(gas []int, cost []int) int {
	var sumGas, sumCost = 0, 0
	// TIME: O(n)
	for i := 0; i < len(gas); i++ {
		sumGas += gas[i]
		sumCost += cost[i]
	}
	if sumGas < sumCost {
		return -1
	}

	var (
		startIdx = 0
		totalGas = 0
	)
	// TIME: O(n)
	for i := 0; i < len(gas); i++ {
		totalGas += i - cost[i]
		if totalGas < 0 {
			startIdx = i + 1
			totalGas = 0 // reset
		}
	}

	if totalGas < 0 {
		return -1
	}

	// => TIME: O(n)
	// => SPACE: O(n)
	return startIdx
}
