package gosol

import (
	"container/heap"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func mergeKLists(lists []*ListNode) *ListNode {
	m := new(minHeap)
	heap.Init(m)
	dummy := new(ListNode)

	for i := range lists {
		if lists[i] == nil {
			continue
		}
		heap.Push(m, lists[i])
	}

	curr := dummy
	for m.Len() != 0 {
		node := heap.Pop(m).(*ListNode)
		curr.Next = node
		if node.Next != nil {
			heap.Push(m, node.Next)
		}
		curr = curr.Next
	}

	return dummy.Next
}

type minHeap []*ListNode

func (m minHeap) Len() int {
	return len(m)
}

func (m minHeap) Less(i, j int) bool {
	return m[i].Val < m[j].Val
}

func (m *minHeap) Swap(i, j int) {
	(*m)[i], (*m)[j] = (*m)[j], (*m)[i]
}

func (m *minHeap) Push(x any) {
	(*m) = append((*m), x.(*ListNode))
}

func (m *minHeap) Pop() any {
	l := m.Len()
	v := (*m)[l-1]
	(*m) = (*m)[:l-1]

	return v
}
