package gosol

import "math"

/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func minDepthDFS(root *TreeNode) int {
	if root == nil {
		return 0
	}

	return dfs(root, 1)
}

func dfs(node *TreeNode, depth int) int {
	if node.Left == nil && node.Right == nil { // found left
		return depth
	}

	var d = math.MaxInt
	if node.Left != nil {
		d = dfs(node.Left, depth+1)
	}
	if node.Right != nil {
		d = min(d, dfs(node.Right, depth+1))
	}

	return d
}
