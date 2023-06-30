package mediumlowestcommonancestorofabinarysearchtree

func lowestCommonAncestor2(root, p, q *TreeNode) *TreeNode {
	pVal, qVal := p.Val, q.Val

	// because it is guranteed to find an answer -> doesnt need a guard condition
	for {
		switch {
		// this is a binary tree, if p, q are bigger than the current node, they must be on the right side
		case pVal > root.Val && qVal > root.Val:
			root = root.Right
		// if p, q are smaller, they must be on the left side
		case pVal < root.Val && qVal < root.Val:
			root = root.Left
		default:
			// p == curr || q == curr -> can not split anymore -> LCA
			// (p > curr && q < curr) || (p < curr && q > curr) -> split point -> LCA
			return root
		}
	}
}
