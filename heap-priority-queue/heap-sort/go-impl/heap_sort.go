package goimpl

func Heapify(s []int, idx int) {
	if idx < 0 || idx >= len(s) {
		return
	}

	var (
		largestIdx    = idx
		leftChildIdx  = idx*2 + 1
		rightChildIdx = idx*2 + 2
	)

	// find the largest index between a node at index [idx] and its child
	if leftChildIdx < len(s) && s[leftChildIdx] > s[largestIdx] {
		largestIdx = leftChildIdx
	}
	if rightChildIdx < len(s) && s[rightChildIdx] > s[largestIdx] {
		largestIdx = rightChildIdx
	}

	// if its child is greater than the node
	if largestIdx != idx {
		// swap with the largest
		s[idx], s[largestIdx] = s[largestIdx], s[idx]

		// heapify the children tree
		Heapify(s, largestIdx)
	}
}

func HeapSort(s []int) {
	// build heap (start from a non-left node): idx = n/2 - 1, idx >= n/2 -1 -> left
	// n/2 - 1: has at least a child
	// 	(n/2 - 1) * 2 + 1 = n - 2 + 1 = n - 1
	// 	(n/2 - 1) * 2 + 2 = n - 2 + 2 = n
	length := len(s)
	for i := length/2 - 1; i >= 0; i-- {
		Heapify(s, i)
	}

	// sorting (asc)
	for i := length - 1; i >= 0; i-- {
		// swap the largest (root) to last
		s[0], s[i] = s[i], s[0]

		// rebuild heap from the remain
		Heapify(s[:i], 0)
	}
}
