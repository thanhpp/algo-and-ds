package gosol

// Definition for a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 || len(inorder) == 0 {
		return nil
	}
	// fmt.Println("buildTree", "preorder", preorder, "inorder", inorder)

	// the root node of a sub-tree -> first value of the pre-order traversal
	val := preorder[0]
	node := &TreeNode{
		Val: val,
	}

	// split the in-order
	// values on the left side of the root value is in the left tree, so on the right side
	idx := -1
	for i := 0; i < len(inorder); i++ {
		// fmt.Println("i", i, "inorder[i]", inorder, "val", val)
		if inorder[i] == val {
			idx = i // how many number are in the left sub tree
			break
		}
	}
	// also remove the node value from each inorder list
	preorderLeft := preorder[1 : idx+1]
	inorderLeft := inorder[:idx]
	preorderRight := preorder[idx+1:]
	inorderRight := inorder[idx+1:]
	node.Left = buildTree(preorderLeft, inorderLeft)
	node.Right = buildTree(preorderRight, inorderRight)

	return node
}
