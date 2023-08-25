package gosol

func subsetsIter(nums []int) [][]int {
	var results [][]int = [][]int{{}}

	for _, n := range nums {
		s := len(results)
		for i := 0; i < s; i++ {
			var subset = make([]int, len(results[i]))
			copy(subset, results[i])
			results = append(results, subset)
			results[len(results)-1] = append(results[len(results)-1], n)
		}
	}

	return results
}

/*
[1, 2, 3]

results = [[]]

- n = 1, s = 1
    - i = 0
        - results = [[], []] (copy results[i] & append to last)
        - results = [[], [1]] (append to last subset)
- n = 2, s = 2
    - i = 0
        - results = [[], [1], []] (copy results[i] & append to last)
        - results = [[], [1], [2]] (append to last subset)
    - i = 1
        - results = [[], [1], [2], [1]] (copy results[i] & append to last)
        - results = [[], [1], [2], [1, 2]] (append to last subset)
- n = 3, s = 4
    - i = 0
        - results = [[], [1], [2], [1, 2], []] (copy results[i] & append to last)
        - results = [[], [1], [2], [1, 2], [3]] (append to last subset)
    - i = 1
        - results = [[], [1], [2], [1, 2], [3], [1]] (copy results[i] & append to last)
        - results = [[], [1], [2], [1, 2], [3], [1, 3]] (append to last subset)
    - i = 2
        - results = [[], [1], [2], [1, 2], [3], [1, 3], [2]] (copy results[i] & append to last)
        - results = [[], [1], [2], [1, 2], [3], [1, 3], [2, 3]] (append to last subset)
    - i = 3
        - results = [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2]] (copy results[i] & append to last)
        - results = [[], [1], [2], [1, 2], [3], [1, 3], [2, 3], [1, 2, 3]] (append to last subset)
*/
