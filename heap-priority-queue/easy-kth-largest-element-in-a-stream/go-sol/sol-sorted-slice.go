package gosol

import (
	"sort"
)

type KthLargestSortedSlice struct {
	k    int
	data []int
}

func ConstructorKthLargestSortedSlice(k int, nums []int) KthLargestSortedSlice {
	return KthLargestSortedSlice{
		k:    k,
		data: nums,
	}
}

func (this *KthLargestSortedSlice) Add(val int) int {
	this.data = append(this.data, val)
	sort.Slice(this.data, func(i, j int) bool {
		return this.data[i] > this.data[j]
	})

	return this.data[this.k-1]
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */
