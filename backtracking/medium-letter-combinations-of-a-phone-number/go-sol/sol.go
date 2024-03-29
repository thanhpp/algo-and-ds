package gosol

var lettersMapping = map[byte][]byte{
	'2': {'a', 'b', 'c'},
	'3': {'d', 'e', 'f'},
	'4': {'g', 'h', 'i'},
	'5': {'j', 'k', 'l'},
	'6': {'m', 'n', 'o'},
	'7': {'p', 'q', 'r', 's'},
	'8': {'t', 'u', 'v'},
	'9': {'w', 'x', 'y', 'z'},
}

func letterCombinations(digits string) []string {
	if len(digits) == 0 {
		return nil
	}

	var (
		res       []string
		backtrack func(d []byte, comb []byte)
	)

	backtrack = func(d []byte, comb []byte) {
		if len(d) <= 0 {
			res = append(res, string(comb))
			return
		}

		letters := lettersMapping[d[0]]
		for _, l := range letters {
			comb = append(comb, l)
			backtrack(d[1:], comb)
			comb = comb[:len(comb)-1]
		}
	}

	backtrack([]byte(digits), make([]byte, 0, len(digits)))

	return res
}
