package gosol

import "math"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 3ms, 5.1MB
func isValidBST(root *TreeNode) bool {
	return isValid(root.Left, root.Val, math.MinInt) &&
		isValid(root.Right, math.MaxInt, root.Val)
}

func isValid(node *TreeNode, max, min int) bool {
	if node == nil {
		return true
	}

	if node.Val >= max || node.Val <= min {
		return false
	}

	return isValid(node.Left, node.Val, min) && isValid(node.Right, max, node.Val)
}
