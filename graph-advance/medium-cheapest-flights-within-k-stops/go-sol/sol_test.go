package gosol

import "testing"

func Test1(t *testing.T) {
	/*
		Input: n = 4, flights = [[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]], src = 0, dst = 3, k = 1
		Output: 700
	*/
	r := findCheapestPrice(
		4,
		[][]int{{0, 1, 100}, {1, 2, 100}, {2, 0, 100}, {1, 3, 600}, {2, 3, 200}},
		0,
		3,
		1,
	)
	t.Log(r)
}
