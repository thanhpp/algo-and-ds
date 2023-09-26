package gosol

func maxAreaOfIsland(grid [][]int) int {
	var (
		maxArea    = 0
		calArea    func(r, c int, area *int)
		rows, cols = len(grid), len(grid[0])
	)

	calArea = func(r, c int, area *int) {
		if grid[r][c] != 1 {
			return
		}

		// increase area
		*area += 1
		if *area > maxArea {
			maxArea = *area
		}

		// mark as visited
		grid[r][c] = 2

		// visit next
		if r > 0 {
			calArea(r-1, c, area)
		}
		if r < rows-1 {
			calArea(r+1, c, area)
		}
		if c > 0 {
			calArea(r, c-1, area)
		}
		if c < cols-1 {
			calArea(r, c+1, area)
		}
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] == 1 {
				area := 0
				calArea(r, c, &area)
			}
		}
	}

	return maxArea
}
