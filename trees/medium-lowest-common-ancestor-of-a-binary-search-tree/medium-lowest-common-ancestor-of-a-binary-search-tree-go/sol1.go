package mediumlowestcommonancestorofabinarysearchtree

// Definition for a binary tree node
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 20ms, 6.9MB
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	// traverse tree
	// set up queue
	var queue []*TreeNode
	queue = append(queue, root)

	var lca *TreeNode
	// bfs
	for {
		l := len(queue)
		if l == 0 {
			break
		}

		// pop from queue
		for i := 0; i < l; i++ {
			pop := queue[0]
			queue = queue[1:]

			// not a common ancestor
			if !bfsIsContains(pop, p, q) {
				continue
			}

			lca = pop

			if pop.Left != nil {
				queue = append(queue, pop.Left)
			}
			if pop.Right != nil {
				queue = append(queue, pop.Right)
			}

		}
	}

	return lca
}

func bfsIsContains(n, p, q *TreeNode) bool {
	if n == nil || p == nil || q == nil {
		return false
	}

	var (
		isPExist, isQExist = false, false
	)

	if n.Val == p.Val {
		isPExist = true
	}
	if n.Val == q.Val {
		isQExist = true
	}

	// set up queue
	var queue []*TreeNode
	queue = append(queue, n)

	// bfs
	for {
		l := len(queue)
		if l == 0 {
			break
		}

		// pop from queue
		for i := 0; i < l; i++ {
			pop := queue[0]
			queue = queue[1:]

			if pop.Val == p.Val {
				isPExist = true
			}
			if pop.Val == q.Val {
				isQExist = true
			}

			if isPExist && isQExist {
				return true
			}

			if pop.Left != nil {
				queue = append(queue, pop.Left)
			}
			if pop.Right != nil {
				queue = append(queue, pop.Right)
			}
		}
	}

	return isPExist && isQExist
}
