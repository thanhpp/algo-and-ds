package gosol

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 12ms, 6.4MB
func kthSmallest(root *TreeNode, k int) int {
	idx := 0
	result := -1
	dfs(root, k, &idx, &result)

	return result
}

func dfs(node *TreeNode, k int, idx, result *int) {
	if node == nil || *result != -1 {
		return
	}

	if node.Left == nil {
		*idx += 1
	}
	if node.Left != nil {
		dfs(node.Left, k, idx, result)
		*idx += 1
	}
	if k == *idx {
		*result = node.Val
		return
	}

	dfs(node.Right, k, idx, result)
}
