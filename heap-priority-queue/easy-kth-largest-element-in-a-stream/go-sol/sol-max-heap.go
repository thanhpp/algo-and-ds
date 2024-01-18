package gosol

import "container/heap"

type KthLargest struct {
	k int
	// minHeap maintains only k largest elements
	minHeap IntHeap
}

// An IntHeap is a min-heap of ints.
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	// Push and Pop use pointer receivers because they modify the slice's length,
	// not just its contents.
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func (h *IntHeap) Peek() any {
	return (*h)[0]
}

func Constructor(k int, nums []int) KthLargest {
	kL := KthLargest{
		k: k,
	}

	for _, n := range nums {
		kL.Add(n)
	}

	return kL
}

func (this *KthLargest) Add(val int) int {
	if this.minHeap.Len() < this.k {
		heap.Push(&this.minHeap, val)
		return this.minHeap.Peek().(int)
	}

	if p := this.minHeap.Peek().(int); p > val {
		return p
	}

	heap.Pop(&this.minHeap)
	heap.Push(&this.minHeap, val)

	return this.minHeap.Peek().(int)
}

// 4
// 4 5 8
// 2 4 5 8
// 2 3 4 5 8
// 2 3 4 5 5 8
