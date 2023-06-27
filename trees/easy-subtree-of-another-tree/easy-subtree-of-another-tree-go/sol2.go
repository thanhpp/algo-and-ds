package easysubtreeofanothertreego

// 12ms, 6.9MB
func isSubtree2(root *TreeNode, subRoot *TreeNode) bool {
	if root == nil {
		if subRoot == nil {
			return true
		} else {
			return false
		}
	}

	if isSameTree2(root, subRoot) {
		return true
	}

	return isSubtree2(root.Left, subRoot) || isSubtree2(root.Right, subRoot)
}

func isSameTree2(q, p *TreeNode) bool {
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
