package easyinvertbinarytreego

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 0ms, 2.1 MB
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return root
	}

	invertTree(root.Left)
	invertTree(root.Right)

	root.Left, root.Right = root.Right, root.Left

	return root
}
