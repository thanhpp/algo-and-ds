package gosol

func minDepth(root *TreeNode) int {
	// using bfs to search each level
	// return when a level contains a leaf
	// NOTE: dfs -> need to check all leaf then returns the smallest depth. BFS -> might returns ealier

	if root == nil {
		return 0
	}

	var (
		q     []*TreeNode
		depth = 0
	)
	q = append(q, root)

	for len(q) != 0 {
		l := len(q)
		depth += 1
		for i := 0; i < l; i++ {
			// pop first
			n := q[0]
			q = q[1:]

			if n.Left == nil && n.Right == nil {
				return depth
			}

			if n.Left != nil {
				q = append(q, n.Left)
			}
			if n.Right != nil {
				q = append(q, n.Right)
			}
		}
	}

	return depth
}
