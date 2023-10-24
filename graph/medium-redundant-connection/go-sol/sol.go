package gosol

func findRedundantConnection(edges [][]int) []int {
	n := len(edges) + 1       // the graph has n nodes, skip index 0
	parents := make([]int, n) // parents[i] = parent of node i (not the direct parent)
	for i := 0; i < n; i++ {
		parents[i] = i // a node is a parent of itself
	}
	rank := make([]int, n) // rank[i] = size of the tree that takes node i as the root
	for i := 0; i < n; i++ {
		rank[i] = 1 // each tree has 1 node (the root)
	}

	var (
		find  func(node int) (parent int)
		union func(a, b int) bool
	)
	find = func(node int) (parent int) {
		p := parents[node]

		// find the greater parent
		for p != parents[p] {
			// path compression, record the greater parent
			parents[p] = parents[parents[p]]
			p = parents[p]
		}

		return p
	}

	union = func(a, b int) bool {
		parA, parB := find(a), find(b)

		// same parent -> add this edge will create a cycle
		if parA == parB {
			return false
		}

		// union strategy: merge the tree that has the lower rank to the tree that has the higher rank
		if rank[parA] > rank[parB] {
			// parB becomes a child of parA
			parents[parB] = parA
			rank[parA] += rank[parB]
			return true
		}

		// parA becomes a child of parB
		parents[parA] = parB
		rank[parB] += rank[parA]
		return true
	}

	for _, e := range edges {
		if !union(e[0], e[1]) {
			return e
		}
	}

	return nil
}
