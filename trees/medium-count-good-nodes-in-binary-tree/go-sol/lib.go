package gosol

import "math"

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func goodNodes(root *TreeNode) int {
	result := 0
	dfs(root, math.MinInt, &result)

	return result
}

func dfs(node *TreeNode, maxVal int, result *int) {
	if node == nil {
		return
	}
	if node.Val >= maxVal {
		*result += 1
		maxVal = node.Val
	}
	dfs(node.Left, maxVal, result)
	dfs(node.Right, maxVal, result)
}
