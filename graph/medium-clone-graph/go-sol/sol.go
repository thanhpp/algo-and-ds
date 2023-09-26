package gosol

// Definition for a Node.
type Node struct {
	Val       int
	Neighbors []*Node
}

func cloneGraph(node *Node) *Node {
	if node == nil {
		return nil
	}

	var (
		cloned  = make(map[*Node]*Node) // src -> clone
		cloneFn func(*Node) *Node
	)

	cloneFn = func(src *Node) *Node {
		if clone, ok := cloned[src]; ok {
			return clone
		}

		// clone a new node
		clone := &Node{
			Val:       src.Val,
			Neighbors: make([]*Node, 0, len(src.Neighbors)),
		}
		cloned[src] = clone

		// copy all neighbors
		for _, nb := range src.Neighbors {
			clone.Neighbors = append(clone.Neighbors, cloneFn(nb))
		}

		return clone
	}

	return cloneFn(node)
}
