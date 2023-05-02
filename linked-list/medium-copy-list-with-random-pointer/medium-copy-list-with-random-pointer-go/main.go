package main

// Definition for a Node.
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

// 0ms, 3.5 MB
func copyRandomList(head *Node) *Node {
	newHead := &Node{}
	curr := newHead
	m := make(map[*Node]*Node) // old ptr -> new ptr

	for head != nil {
		// init a new node
		curr.Next = &Node{
			Val:    head.Val,
			Next:   nil,
			Random: head.Random,
		}

		// create old -> new mapping
		m[head] = curr.Next

		// move forward
		curr = curr.Next
		head = head.Next
	}

	// replace the old Random ptr to the new Random ptr
	curr = newHead
	for curr != nil {
		curr.Random = m[curr.Random]
		curr = curr.Next
	}

	return newHead.Next
}
