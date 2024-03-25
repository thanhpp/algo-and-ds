package gosol

func mergeTriplets(triplets [][]int, target []int) bool {
	var filtered = make([][]int, 0)

	// if a triplets contains an element larger than the target
	// it can not be merged, otherwise the larger element will stays in the merged triplet
	for _, t := range triplets {
		var isValid = true
		for i := range t {
			if t[i] > target[i] {
				isValid = false
				break
			}
		}
		if !isValid {
			continue
		}

		filtered = append(filtered, t)
	}
	// filtered contains triplets that only has each element smaller or equal to the same index element in the target

	// find if each element exist in filtered triplets -> all exist -> target can be merged in someway
	found := make([]bool, 3)
	for _, f := range filtered {
		found[0] = found[0] || (f[0] == target[0])
		found[1] = found[1] || (f[1] == target[1])
		found[2] = found[2] || (f[2] == target[2])
	}

	// TIME: O(3n)
	// SPACE: O(n)
	return found[0] && found[1] && found[2]
}

func mergeTripletsOptimized(triplets [][]int, target []int) bool {
	var found = [3]bool{false, false, false}

	for i := range triplets {
		if triplets[i][0] > target[0] || triplets[i][1] > target[1] || triplets[i][2] > target[2] {
			continue
		}

		found[0] = found[0] || triplets[i][0] == target[0]
		found[1] = found[1] || triplets[i][1] == target[1]
		found[2] = found[2] || triplets[i][2] == target[2]

		if found[0] && found[1] && found[2] {
			return true
		}
	}

	return false
}
