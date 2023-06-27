package easysubtreeofanothertreego

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// 22ms, 7.3MB
func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	if root == nil && subRoot == nil {
		return true
	}
	if !(root != nil && subRoot != nil) {
		return false
	}
	// do bfs
	var stack []*TreeNode
	stack = append(stack, root)

	for {
		length := len(stack)
		if length == 0 {
			break
		}
		for i := 0; i < length; i++ {
			currNode := stack[0]
			stack = stack[1:]
			if currNode.Val == subRoot.Val && isSameTree(currNode, subRoot) {
				return true
			}
			if currNode.Left != nil {
				stack = append(stack, currNode.Left)
			}
			if currNode.Right != nil {
				stack = append(stack, currNode.Right)
			}
		}
	}

	return false
}

func isSameTree(q, p *TreeNode) bool {
	if q == nil && p == nil {
		return true
	}
	if !(q != nil && p != nil) {
		return false
	}
	if q.Val != p.Val {
		return false
	}

	return isSameTree(q.Left, p.Left) && isSameTree(q.Right, p.Right)
}
