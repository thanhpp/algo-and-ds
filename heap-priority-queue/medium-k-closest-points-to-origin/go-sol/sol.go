package gosol

import (
	"container/heap"
	"math"
)

type Point struct {
	X    int
	Y    int
	Dist float64
}

type MinHeap []Point

func (h *MinHeap) Len() int {
	return len(*h)
}
func (h *MinHeap) Less(i, j int) bool {
	return (*h)[i].Dist < (*h)[j].Dist
}
func (h *MinHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}
func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(Point))
}
func (h *MinHeap) Pop() any {
	v := (*h)[h.Len()-1]
	(*h) = (*h)[:h.Len()-1]

	return v
}

func kClosest(points [][]int, k int) [][]int {
	var h = new(MinHeap)
	heap.Init(h)

	for _, p := range points {
		dist := math.Sqrt(float64(p[0]*p[0]) + float64(p[1]*p[1]))
		heap.Push(h, Point{
			X:    p[0],
			Y:    p[1],
			Dist: dist,
		})
	}

	resp := make([][]int, 0, k)
	for i := 0; i < k; i++ {
		p := heap.Pop(h).(Point)
		resp = append(resp, []int{p.X, p.Y})
	}

	return resp
}
