package gosol

import "container/heap"

type MinHeap []int

func (h MinHeap) Len() int {
	return len(h)
}

func (h MinHeap) Less(i, j int) bool {
	return h[i] > h[j]
}

func (h *MinHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

// add x as element Len()
func (h *MinHeap) Push(x any) {
	(*h) = append((*h), x.(int))
}

// remove and return element Len() - 1.
func (h *MinHeap) Pop() any {
	i := h.Len() - 1
	v := (*h)[i]
	(*h) = (*h)[:i]

	return v
}

func (h MinHeap) Peek() any {
	return h[0]
}

func isNStraightHand(hand []int, groupSize int) bool {
	// first check
	if len(hand)%groupSize != 0 {
		return false
	}

	// create a heap to check for minimum values
	minHeap := new(MinHeap)
	heap.Init(minHeap)

	// count unique hands
	count := make(map[int]int) // hand -> count
	for _, h := range hand {
		if _, ok := count[h]; !ok {
			heap.Push(minHeap, h)
		}

		count[h] += 1
	}

	// always start from the minimum value
	for minHeap.Len() != 0 {
		start := heap.Pop(minHeap).(int)
		for i := 0; i < groupSize; i++ {
			v := start + i
			c, ok := count[v]
			if !ok {
				return false
			}
			if c <= 0 {
				return false
			}
			count[v] -= 1
			if count[v] == 0 {
				// there is no more v -> if it is true, no more of the smallest value
				minVal := heap.Pop(minHeap).(int)
				if minVal != v {
					return false
				}
			}
		}

	}

	return true
}
