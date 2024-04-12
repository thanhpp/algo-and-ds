package gosol

import "slices"

func minCostConnectPoints(points [][]int) int {
	// build adj list (no direction) - TIME: O(n^2), SPACE: O(n^2)
	edges := make([]Edge, 0, 0)
	for i := range points {
		for j := i + 1; j < len(points); j++ {
			p1 := [2]int{points[i][0], points[i][1]}
			p2 := [2]int{points[j][0], points[j][1]}
			dist := calManhattanDistance(p1, p2)

			// add to map
			edges = append(edges,
				Edge{
					FromIdx: i,
					ToIdx:   j,
					Weight:  dist,
				})
		}
	}

	// SORT EDGES - O(nlogn)
	slices.SortFunc[[]Edge](edges, func(a, b Edge) int {
		if a.Weight < b.Weight {
			return -1
		}
		if a.Weight == b.Weight {
			return 0
		}
		return 1
	})

	// init find-union structures to check for loop
	parents := make([]int, len(points))
	ranks := make([]int, len(points))
	for i := range points {
		parents[i] = i
	}

	// Kruskal
	var kEdges []Edge
	for _, e := range edges {
		// check for loop
		if findParents(parents, e.FromIdx) == findParents(parents, e.ToIdx) {
			continue
		}

		// add to kEdges
		kEdges = append(kEdges, e)
		// union
		unions(parents, ranks, e.FromIdx, e.ToIdx)
	}

	// Sum minimum spanning tree
	var sum = 0
	for _, e := range kEdges {
		sum += e.Weight
	}

	return sum
}

type Edge struct {
	FromIdx int
	ToIdx   int
	Weight  int
}

func calManhattanDistance(a, b [2]int) int {
	return abs(a[0]-b[0]) + abs(a[1]-b[1])
}

func abs(a int) int {
	if a < 0 {
		return -a
	}

	return a
}

func findParents(parents []int, childIdx int) int {
	if parents[childIdx] == childIdx { // parent of itself
		return childIdx
	}

	p := findParents(parents, parents[childIdx])
	parents[childIdx] = p // shorten find path

	return p
}

func unions(parents []int, ranks []int, x, y int) {
	pX := findParents(parents, x)
	pY := findParents(parents, y)

	if pX == pY {
		return
	}

	if ranks[pX] < ranks[pY] {
		parents[pX] = pY
		return
	}
	if ranks[pX] > ranks[pY] {
		parents[pY] = pX
		return
	}

	parents[pX] = pY
	ranks[pY] += 1
}
