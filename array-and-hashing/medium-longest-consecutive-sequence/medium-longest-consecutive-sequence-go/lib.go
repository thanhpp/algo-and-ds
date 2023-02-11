package main

import "sort"

// 83ms, 10.1MB
func longestConsecutive(nums []int) int {
	m := make(map[int][]int) // largest

	sort.Ints(nums) // O(nlogn)

	// O(n)
	for _, n := range nums {
		if s, ok := m[n-1]; ok {
			s = append(s, n)
			delete(m, n-1)
			m[n] = s
			continue
		}
		if s, ok := m[n]; ok {
			s = append(s, n)
			m[n] = s
			continue
		}
		m[n] = []int{n}
	}

	// o()
	var max = 0
	for i := range m {
		if len(m[i]) > max {
			max = len(m[i])
		}
	}

	return max
}
