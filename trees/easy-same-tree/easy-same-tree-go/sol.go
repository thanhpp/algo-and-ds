package easysametreego

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSameTree(p *TreeNode, q *TreeNode) bool {
	switch {
	case p == nil && q == nil:
		return true
	case (p != nil && q == nil) || (p == nil && q != nil):
		return false
	case p.Val != q.Val:
		return false
	default:
		return isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
	}
}
