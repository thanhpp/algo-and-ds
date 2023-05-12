package easylinkedlistcyclego

type ListNode struct {
	Val  int
	Next *ListNode
}

// check if is there a duplicate pointer
func hasCycle(head *ListNode) bool {
	m := make(map[*ListNode]struct{})
	m[head] = struct{}{}
	curr := head
	for curr != nil {
		curr = curr.Next
		if _, ok := m[curr]; ok {
			return true
		}
		m[curr] = struct{}{}
	}

	return false
}
