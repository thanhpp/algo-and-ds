package gosol

func numIslands(grid [][]byte) int {
	var (
		island int
		stack  [][2]int // (row, col)
		rows   = len(grid)
		cols   = len(grid[0])
	)

	// find first land of an island
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] != '1' { // not an island or visited
				continue
			}
			stack = append(stack, [2]int{r, c})
			island++ // found an island
			// dfs to mark all land of an island
			for len(stack) != 0 {
				last := stack[len(stack)-1]
				stack = stack[:len(stack)-1] // pop last
				grid[last[0]][last[1]] = 'x' // mark as visited

				// add vertical
				if nextR, nextC := last[0]-1, last[1]; nextR >= 0 && grid[nextR][nextC] == '1' {
					stack = append(stack, [2]int{nextR, nextC})
				}
				if nextR, nextC := last[0]+1, last[1]; nextR < rows && grid[nextR][nextC] == '1' {
					stack = append(stack, [2]int{nextR, nextC})
				}
				// add horizontal
				if nextR, nextC := last[0], last[1]-1; nextC >= 0 && grid[nextR][nextC] == '1' {
					stack = append(stack, [2]int{nextR, nextC})
				}
				if nextR, nextC := last[0], last[1]+1; nextC < cols && grid[nextR][nextC] == '1' {
					stack = append(stack, [2]int{nextR, nextC})
				}
			}
		}
	}

	return island
}
