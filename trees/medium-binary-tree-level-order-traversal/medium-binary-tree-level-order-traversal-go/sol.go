package mediumbinarytreelevelordertraversalgo

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func levelOrder(root *TreeNode) [][]int {
	if root == nil {
		return nil
	}

	order := make([][]int, 0)
	q := make([]*TreeNode, 0)
	q = append(q, root)

	for {
		l := len(q)
		if l == 0 {
			break
		}

		level := make([]int, 0, l)
		for i := 0; i < l; i++ {
			node := q[0]
			q = q[1:]

			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}

			level = append(level, node.Val)
		}

		order = append(order, level)
	}

	return order
}
