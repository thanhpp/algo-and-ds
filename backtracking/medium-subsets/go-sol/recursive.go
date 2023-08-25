package gosol

func subsets(nums []int) [][]int {
	var result [][]int

	dfs(nums, 0, []int{}, &result)

	return result
}

func dfs(nums []int, idx int, subset []int, result *[][]int) {
	// reached the last step of the decision tree
	if idx == len(nums) {
		*result = append(*result, subset)
		// fmt.Println("appended", subset, "result", result)
		return
	}

	// not add the current value to the subset
	dfs(nums, idx+1, subset, result)

	// add the current value to the subset
	subset = append(subset, nums[idx])
	var addSubset = make([]int, len(subset))
	copy(addSubset, subset)
	dfs(nums, idx+1, addSubset, result)
}

/*
nums = [1, 2]

            []
            /\
           /  \
          /    \
         /      \
        []       1           (add 1 or not)
        /\       /\
       /  \     /  \
      []   2   []   2        (add 2 or not)
=>    []   2    1   1,2
*/
