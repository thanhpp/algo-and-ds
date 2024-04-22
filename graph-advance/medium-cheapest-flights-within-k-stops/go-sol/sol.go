package gosol

import (
	"container/heap"
	"math"
)

/*
CONCEPT: USING DIJIKSTRA WITH HOPS RECORDED
- At each vertex, record the minimum cost to reach it by (0..k) hops
- Find the minimum value in the dest (0..k) hops costs
*/
func findCheapestPrice(n int, flights [][]int, src int, dst int, k int) int {
	// create adj list, index[i] -> flights from i to anywhere
	adjList := make([][][3]int, n)
	for _, f := range flights {
		from := f[0]
		to := f[1]
		price := f[2]
		adjList[from] = append(adjList[from], [3]int{from, to, price})
	}

	// prepare data structure to run dijikstra
	var distances = make([][]int, n) // distances[i][j] = min cost to reach i using j hops
	for i := range distances {
		distances[i] = make([]int, k+2) // k stops between -> k + 1 hops
		for j := range distances[i] {
			distances[i][j] = math.MaxInt
		}
	}
	var isOptimized = make(map[[2]int]bool)
	var minHeap = new(MinHeap)
	heap.Init(minHeap)

	// insert pre-data
	heap.Push(minHeap, Node{
		To:         src,
		TotalPrice: 0,
		Hop:        0,
	})

	for minHeap.Len() != 0 {
		n := heap.Pop(minHeap).(Node)
		if isOptimized[[2]int{n.To, n.Hop}] {
			continue
		}
		isOptimized[[2]int{n.To, n.Hop}] = true

		if distances[n.To][n.Hop] > n.TotalPrice {
			distances[n.To][n.Hop] = n.TotalPrice

			if n.Hop == k+1 {
				continue // reached max hop
			}

			for _, f := range adjList[n.To] {
				heap.Push(minHeap, Node{
					To:         f[1],
					TotalPrice: n.TotalPrice + f[2],
					Hop:        n.Hop + 1,
				})
			}
		}
	}

	var cheapest = math.MaxInt
	for _, v := range distances[dst] {
		cheapest = min(cheapest, v)
	}

	// fmt.Println(distances)

	if cheapest == math.MaxInt {
		return -1
	}

	return cheapest
}

type MinHeap []Node

type Node struct {
	To         int
	TotalPrice int
	Hop        int
}

func (h MinHeap) Len() int {
	return len(h)
}

func (h MinHeap) Less(i, j int) bool {
	return h[i].TotalPrice < h[j].TotalPrice
}

func (h *MinHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

func (h *MinHeap) Push(x any) {
	(*h) = append((*h), x.(Node))
}

func (h *MinHeap) Pop() any {
	if h.Len() == 0 {
		return nil
	}

	v := (*h)[h.Len()-1]
	(*h) = (*h)[:h.Len()-1]

	return v
}
