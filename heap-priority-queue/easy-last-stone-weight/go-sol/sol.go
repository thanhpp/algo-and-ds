package gosol

import (
	"container/heap"
	"math"
)

func lastStoneWeight(stones []int) int {
	var m = MaxHeap(stones)
	heap.Init(&m)

	for m.Len() > 1 {
		// fmt.Println(m)

		first := heap.Pop(&m).(int)
		second := heap.Pop(&m).(int)

		// fmt.Println(first, second)

		left := int(math.Abs(float64(first - second)))
		if left == 0 {
			continue
		}
		heap.Push(&m, left)
	}

	if m.Len() == 0 {
		return 0
	}

	return heap.Pop(&m).(int)
}

type MaxHeap []int

func (h *MaxHeap) Len() int {
	return len(*h)
}

func (h *MaxHeap) Less(i, j int) bool {
	return (*h)[i] > (*h)[j] // max  heap
}

func (h *MaxHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

func (h *MaxHeap) Push(x any) {
	(*h) = append((*h), x.(int))
}

func (h *MaxHeap) Pop() any {
	l := len(*h)
	v := (*h)[l-1]

	(*h) = (*h)[:l-1]

	return v
}
