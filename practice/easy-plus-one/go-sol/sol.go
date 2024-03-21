package gosol

func plusOne(digits []int) []int {
	res := make([]int, len(digits)+1)
	remain := 1
	for i := len(digits) - 1; i >= 0; i-- {
		val := digits[i] + remain
		if val >= 10 {
			val -= 10
		} else {
			remain = 0
		}
		res[i+1] = val
	}
	if remain == 1 {
		res[0] = 1
		return res
	}
	return res[1:]
}
