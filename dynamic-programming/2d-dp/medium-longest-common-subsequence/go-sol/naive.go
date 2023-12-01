package gosol

import "math"

func longestCommonSubsequenceNaive(text1 string, text2 string) int {
	sub1, sub2 := Subsequences(text1), Subsequences(text2)

	var maxLength int
	for k := range sub1 {
		if _, ok := sub2[k]; !ok {
			continue
		}

		if l := len(k); l > maxLength {
			maxLength = l
		}
	}

	return maxLength
}

func Subsequences(text string) map[string]bool {
	m := make(map[string]bool, int(math.Pow(2, float64(len(text)))))

	subsequencesDFS(text, 0, "", m)

	return m
}

func subsequencesDFS(text string, idx int, path string, set map[string]bool) {
	if idx >= len(text) {
		return
	}

	subsequencesDFS(text, idx+1, path, set)

	path += string(text[idx])
	if _, ok := set[path]; !ok {
		set[path] = true
	}

	subsequencesDFS(text, idx+1, path, set)
}
