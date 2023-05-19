package lru

// 573ms, 73MB
type node struct {
	key    int
	val    int
	parent *node
	child  *node
}

type LRUCache struct {
	capacity int
	data     map[int]*node // key -> node
	head     *node
	tail     *node
}

func Constructor(capacity int) LRUCache {
	return LRUCache{
		capacity: capacity,
		data:     make(map[int]*node, capacity),
		// dummy head
		head: &node{
			key: -1,
			val: -1,
		},
	}
}

func (c *LRUCache) DEBUG() {
	// fmt.Printf("HEAD:  %d | %v \n", c.head.key, c.head)
	// fmt.Printf("TAIL:  %d | %v \n", c.tail.key, c.tail)

	curr := c.head
	for curr != nil {
		// fmt.Printf("addr: %p, key: %d, val: %d, before: %p, next: %p \n", curr, curr.key, curr.val, curr.parent, curr.child)
		curr = curr.child
	}
}

func (this *LRUCache) Get(key int) int {
	// fmt.Printf("\n GET Start: %d \n", key)

	v, ok := this.data[key]
	if !ok {
		return -1
	}

	// move the key to the head
	if this.head.child == v {
		// fmt.Println("already the head", this.head.child)
		return v.val
	}

	// swap the node
	// update if v is the tail
	if this.tail == v {
		// fmt.Println("old tail", this.tail)
		if v.child != nil {
			this.tail = v.child
		} else {
			this.tail = v.parent
		}
		// fmt.Println("new tail", this.tail)
	}

	// append the new node to the head
	// 1. remove the node references
	v.parent.child = v.child
	if v.child != nil {
		v.child.parent = v.parent
	}
	// 2. update the old head
	this.head.child.parent = v
	// 3. update the new head
	v.parent = this.head
	v.child = this.head.child
	// 4. set the new head
	this.head.child = v
	// fmt.Println("new head", this.head.child)
	// fmt.Println("curr tail", this.tail)
	// this.DEBUG()

	return v.val
}

func (this *LRUCache) Put(key int, value int) {
	// fmt.Printf("\n PUT Start: %d\n", key)
	// handle if the key exist
	n, ok := this.data[key]
	if ok {
		n.val = value
		this.Get(key)
		return
	}

	// check if evict is necessary (not ok)
	if !ok && len(this.data) == this.capacity {
		tail := this.tail
		// fmt.Println("evict start", tail)
		tail.parent.child = tail.child
		if tail.child != nil {
			this.tail = tail.child
		} else {
			this.tail = tail.parent
		}
		delete(this.data, tail.key)
		// fmt.Printf("evict end - deleted: %d - new tail: %v\n", tail.key, this.tail)
	}

	// create a new node
	n = &node{
		key:    key,
		val:    value,
		parent: this.head,
		child:  this.head.child,
	}
	if isFirst := len(this.data) == 0; isFirst {
		this.data[key] = n
		// fmt.Println("first node", this.head.child, this.tail)
		this.head.child = n
		this.tail = n
		return
	}
	this.data[key] = n

	// dummy -> new head -> old head
	//  |         ^ |           ^
	//  | _ _ _ _ | | _ _ _ _ _ |
	// update the old head
	this.head.child.parent = n
	// update the new head
	this.head.child = n
	// fmt.Println("new head", this.head.child)

	// this.DEBUG()
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */
