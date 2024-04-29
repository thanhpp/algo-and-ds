package gosol

import (
	"container/heap"
	"math"
)

func networkDelayTime(times [][]int, n int, k int) int {
	adj := make([][][2]int, n)
	for _, t := range times {
		from, to, cost := t[0]-1, t[1]-1, t[2]
		adj[from] = append(adj[from], [2]int{to, cost})
	}

	var (
		cost      = make([]int, n)
		optimized = make([]bool, n)
		m         = new(minHeap)
	)
	for i := range cost {
		cost[i] = math.MaxInt
	}
	heap.Init(m)
	heap.Push(m, Node{
		from: -1,
		to:   k - 1,
		cost: 0,
	})
	for m.Len() != 0 {
		node := heap.Pop(m).(Node)
		if optimized[node.to] {
			continue
		}
		optimized[node.to] = true

		if node.cost < cost[node.to] {
			cost[node.to] = node.cost
			for _, e := range adj[node.to] {
				heap.Push(m, Node{
					from: node.to,
					to:   e[0],
					cost: node.cost + e[1],
				})
			}
		}
	}

	maxTime := -1
	for _, c := range cost {
		maxTime = max(maxTime, c)
	}
	if maxTime == math.MaxInt {
		return -1
	}

	return maxTime
}

type Node struct {
	from int
	to   int
	cost int
}

type minHeap []Node

func (m *minHeap) Push(x any) {
	(*m) = append((*m), x.(Node))
}

func (m *minHeap) Pop() any {
	l := m.Len()
	if l == 0 {
		return nil
	}

	v := (*m)[l-1]
	(*m) = (*m)[:l-1]

	return v
}

func (m minHeap) Len() int {
	return len(m)
}

func (m minHeap) Less(i, j int) bool {
	return m[i].cost < m[j].cost
}

func (m *minHeap) Swap(i, j int) {
	(*m)[i], (*m)[j] = (*m)[j], (*m)[i]
}
