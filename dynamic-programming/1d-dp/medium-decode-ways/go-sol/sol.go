package gosol

func numDecodings(s string) int {
	// can not decode if there is a leading zero
	if s[0] == '0' {
		return 0
	}

	var count0, count1 = 1, 0 // Space = O(1)

	/*
		dp[i] = - dp[i - 1] + dp[i - 2] (s[i] & s[i+1] can decode 2 digits)
				|
				- dp[i - 1] (s[i] is zero OR can only decode 1 digit)
	*/

	for i := len(s) - 1; i >= 0; i-- { // Time = O(n)
		if s[i] == '0' {
			tmp := 0
			count1 = count0
			count0 = tmp
			continue
		}

		tmp := count0
		if ((i + 1) < len(s)) && ((s[i] == '1') || s[i] == '2' && s[i+1] >= '0' && s[i+1] <= '6') {
			tmp += count1
		}
		count1 = count0
		count0 = tmp
	}

	return count0
}
