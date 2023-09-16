package gosol

func partition(s string) [][]string {
	var (
		path   []string
		result [][]string
	)

	backtrack(s, 0, path, &result)

	return result
}

func backtrack(s string, split int, path []string, result *[][]string) {
	if split >= len(s) {
		tmp := make([]string, len(path))
		copy(tmp, path)
		*result = append(*result, tmp)
		return
	}

	for i := split; i < len(s); i++ {
		next := s[split : i+1]
		if !isPalidrome(next) {
			continue
		}
		path = append(path, next)
		backtrack(s, i+1, path, result)
		path = path[:len(path)-1]
	}
}

func isPalidrome(s string) bool {
	i, j := 0, len(s)-1

	for i <= j {
		if s[i] != s[j] {
			return false
		}
		i++
		j--
	}

	return true
}
