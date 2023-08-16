package gosol

func kthSmallestIter(root *TreeNode, k int) int {
	var (
		idx   = 0
		stack = []*TreeNode{}
		curr  = root
	)

	for curr != nil || len(stack) != 0 {
		// go to the left most
		for curr != nil {
			stack = append(stack, curr)
			curr = curr.Left
		}

		// pop
		curr = stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		idx += 1
		if idx == k {
			return curr.Val
		}

		// visit right side of the left most node
		curr = curr.Right
	}

	return -1
}
