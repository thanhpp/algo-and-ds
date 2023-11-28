package gosol

func uniquePaths(m int, n int) int {
	/*
	   Concept: the number of paths the robot can reach to a position
	   is the sum of the number of paths above and left to it

	   - The robot can only goes right and down -> there are only 1 path to reach the top and leftmost     positions
	   => |  1  |  1  |  1  |  1  |  1   |
	      |  1  |  x  |  x  |  x  |  x   |
	      |  1  |  x  |  x  |  x  |  x   |
	   => need to calculate each row from left to right, top to bottom
	*/

	// TIME = O(n * m)
	// SPACE = O(n * m)

	// create map
	var dp = make([][]int, m)
	for i := range dp { // O(m)
		dp[i] = make([]int, n)
		dp[i][0] = 1 // base case: dp[0..m][0] = 1
	}

	// base case dp[0][0..n] = 1
	for i := 0; i < n; i++ { // O(n)
		dp[0][i] = 1
	}

	// dp[i][j] = dp[i-1] + dp[j-1] (only right or down)
	for i := 1; i < n; i++ { // O(n)
		for j := 1; j < m; j++ { // O(m)
			dp[j][i] = dp[j-1][i] + dp[j][i-1]
		}
	}

	// fmt.Println(dp)

	return dp[m-1][n-1]
}
