package mediumbinarytreerightsideviewgo

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 3ms, 2.3MB
func rightSideView(root *TreeNode) []int {
	if root == nil {
		return nil
	}

	view := make([]int, 0)
	q := make([]*TreeNode, 0)
	q = append(q, root)

	for {
		l := len(q)
		if l == 0 {
			break
		}

		for i := 0; i < l; i++ {
			node := q[0]
			if i == l-1 {
				view = append(view, node.Val)
			}
			q = q[1:]

			if node.Left != nil {
				q = append(q, node.Left)
			}
			if node.Right != nil {
				q = append(q, node.Right)
			}
		}
	}

	return view
}
