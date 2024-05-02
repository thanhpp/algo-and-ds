package gosol

func isInterleave(s1 string, s2 string, s3 string) bool {
	// must use all s1 & s2 to form s3
	if len(s1)+len(s2) != len(s3) {
		return false
	}

	// dp[i1][i2] = is it possible to use i1..len(s1) & i2..len(s2) to form i1+i2..len(s3)
	// dp[len(s1)][x] <=> s1 == "" (used all s1 - out of bound) ~~ same for s2
	var dp = make([][]bool, len(s1)+1)
	for i := range dp {
		dp[i] = make([]bool, len(s2)+1)
	}

	// add base case
	dp[len(s1)][len(s2)] = true // => used all s1 & s2 -> formed s3

	// filling the dp
	for i1 := len(s1); i1 >= 0; i1-- {
		for i2 := len(s2); i2 >= 0; i2-- {
			i3 := i1 + i2
			c3 := s3[i3]

			if i1 < len(s1) && // bound check
				s1[i1] == c3 && // it is possible to use s1[i1]
				dp[i1+1][i2] /*to use s1[i1] <=> s1[i1 + 1] is used to generate a possible state*/ {
				dp[i1][i2] = true
			}

			if i2 < len(s2) &&
				s2[i2] == c3 &&
				dp[i1][i2+2] {
				dp[i1][i2] = true
			}
		}
	}

	return dp[0][0]
}
