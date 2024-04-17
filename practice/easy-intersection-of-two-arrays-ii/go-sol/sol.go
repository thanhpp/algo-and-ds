package gosol

func intersect(nums1 []int, nums2 []int) []int {
    var (
        nums1Count = make(map[int]int)
        nums2Count = make(map[int]int)
    )
    for _, n := range nums1 {
        nums1Count[n] += 1
    }
    for _, n := range nums2 {
        nums2Count[n] += 1
    }

    var intersect []int

    for n, c1 := range nums1Count {
        c2, ok := nums2Count[n]
        if !ok {
            continue
        }
        for i := 0; i < min(c1, c2); i++ {
            intersect = append(intersect, n)
        }
    }

    return intersect
}