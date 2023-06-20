package easybalancedbinarytreego

import "math"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 3ms, 6MB
func isBalanced(root *TreeNode) bool {
	if root == nil {
		return true
	}

	isBalance, _ := checkBalanced(root)

	return isBalance
}

func checkBalanced(node *TreeNode) (isBalanced bool, height float64) {
	if node == nil {
		return true, 0
	}

	isBalancedLeft, heightLeft := checkBalanced(node.Left)
	isBalancedRight, heightRight := checkBalanced(node.Right)

	if !(isBalancedLeft && isBalancedRight) {
		return false, 0
	}

	if math.Abs(heightLeft-heightRight) > 1 {
		return false, 0
	}

	return true, math.Max(heightLeft, heightRight) + 1
}
