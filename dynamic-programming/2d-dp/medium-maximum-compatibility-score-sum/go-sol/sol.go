package gosol

func maxCompatibilitySum(students [][]int, mentors [][]int) int {
	var maxSum = 0
	dfs(students, mentors, 0, make([]bool, len(mentors)), 0, &maxSum)

	return maxSum
}

func dfs(students, mentors [][]int, sIdx int, usedMentors []bool, current int, maxSum *int) {
	if sIdx >= len(students) {
		if current >= *maxSum {
			*maxSum = current
		}
		return
	}

	for i := range mentors {
		if usedMentors[i] {
			continue
		}

		score := calc(students[sIdx], mentors[i])
		usedMentors[i] = true
		dfs(students, mentors, sIdx+1, usedMentors, current+score, maxSum)
		usedMentors[i] = false
	}
}

func calc(s, m []int) int {
	sum := 0
	for i := range s {
		if s[i] == m[i] {
			sum += 1
		}
	}

	return sum
}
