package gosol

func sumZero(n int) []int {
	res := make([]int, 0, n)
	if n%2 != 0 {
		res = append(res, 0)
		n = n - 1
	}
	for i := 0; i < (n)/2; i++ {
		res = append(res, i+1)
		res = append(res, -(i + 1))
	}

	return res
}
