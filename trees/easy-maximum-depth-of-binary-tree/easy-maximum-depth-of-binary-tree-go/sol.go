package easymaximumdepthofbinarytreego

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 3ms, 4.3MB
func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}

	return depth(root, 1)
}

func depth(node *TreeNode, currDepth int) int {
	if node == nil {
		return currDepth
	}

	left, right := currDepth, currDepth
	if node.Left != nil {
		left++
		left = depth(node.Left, left)
	}
	if node.Right != nil {
		right++
		right = depth(node.Right, right)
	}

	return max(left, right)
}

func max(a, b int) int {
	if a > b {
		return a
	}

	return b
}
