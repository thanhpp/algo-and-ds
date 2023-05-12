package easylinkedlistcyclego

// this uses the tortoise & hare (slow & fast) floyd alogrithm.
// It relies on the fact that if both pointers are moving at a different pace,
// the gap between them will keep on increasing to a limit, after which it'll be reset if a cycle exists.
func hasCycle2(head *ListNode) bool {
	if head.Next == nil {
		return false
	}

	slow, fast := head, head.Next

	for {
		if slow == fast {
			return true
		}

		if slow.Next == nil {
			break
		}
		slow = slow.Next
		if fast.Next == nil || fast.Next.Next == nil {
			break
		}
		fast = fast.Next.Next
	}

	return false
}
