package gosol

func intersect(nums1 []int, nums2 []int) []int {
	res := []int{}
	m := make(map[int]int)

	for _, num := range nums1 {
		m[num]++
	}

	for _, num := range nums2 {
		if counter, ok := m[num]; ok && counter > 0 {
			m[num]--
			res = append(res, num)
            if len(res) == len(nums1) {
                return res
            }
		}
	}

	return res
}