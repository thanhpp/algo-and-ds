package gosol

import "container/heap"

type Task struct {
	Name  byte
	Count int
}

type TaskCountMaxHeap []Task

func (h TaskCountMaxHeap) Len() int {
	return len(h)
}

func (h TaskCountMaxHeap) Less(i, j int) bool {
	return h[i].Count >= h[j].Count
}

func (h *TaskCountMaxHeap) Swap(i, j int) {
	(*h)[i], (*h)[j] = (*h)[j], (*h)[i]
}

func (h *TaskCountMaxHeap) Push(x any) {
	(*h) = append((*h), x.(Task))
}
func (h *TaskCountMaxHeap) Pop() any {
	l := h.Len()
	if l == 0 {
		return nil
	}

	v := (*h)[l-1]
	(*h) = (*h)[:l-1]

	return v
}

type InQueueTask struct {
	Task
	AvailableAt int
}

func leastInterval(tasks []byte, n int) int {
	// count the occurence of each task
	countMap := make(map[byte]int)
	for _, t := range tasks {
		countMap[t] += 1
	}

	maxHeap := new(TaskCountMaxHeap)
	heap.Init(maxHeap)
	for k, v := range countMap {
		heap.Push(maxHeap,
			Task{
				Name:  k,
				Count: v,
			},
		)
	}

	queue := make([]InQueueTask, 0)

	time := 0
	for {
		if maxHeap.Len() == 0 && len(queue) == 0 {
			break
		}

		// add available
		for {
			if len(queue) == 0 || queue[0].AvailableAt > time {
				break
			}
			avail := queue[0]
			queue = queue[1:]
			heap.Push(maxHeap, avail.Task)
		}

		time += 1
		if maxHeap.Len() != 0 {
			t := heap.Pop(maxHeap).(Task)
			if t.Count != 1 {
				t.Count -= 1
				queue = append(queue, InQueueTask{
					Task:        t,
					AvailableAt: time + n,
				})
			}
		}
	}

	return time
}
