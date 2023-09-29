package gosol

// smarter version
// from each node at the border (next to the ocean), traverse in increasing (or equal) order to find which node can
// reach each ocean.
// then check for nodes can reach both of the oceans

// Big(O) = O(2*m*n) ~ O(m*n) - for each ocean, we only need to traverse the graph once

const (
	reachPacific  = 1
	reachAtlantic = 2
	reachBoth     = reachPacific + reachAtlantic
)

func pacificAtlantic(heights [][]int) [][]int {
	var (
		maxRow = len(heights)
		maxCol = len(heights[0])
	)
	var (
		result  [][]int
		visited [][]uint8 = make([][]uint8, maxRow)
	)

	var dfs func(r, c int, ocean uint8)
	dfs = func(r, c int, ocean uint8) {
		if visited[r][c] == reachBoth || visited[r][c] == ocean {
			return
		}
		visited[r][c] += ocean

		if r > 0 && heights[r-1][c] >= heights[r][c] {
			dfs(r-1, c, ocean)
		}
		if r < maxRow-1 && heights[r+1][c] >= heights[r][c] {
			dfs(r+1, c, ocean)
		}
		if c > 0 && heights[r][c-1] >= heights[r][c] {
			dfs(r, c-1, ocean)
		}
		if c < maxCol-1 && heights[r][c+1] >= heights[r][c] {
			dfs(r, c+1, ocean)
		}
	}

	// form visited & its borders
	for r := 0; r < maxRow; r++ {
		visited[r] = make([]uint8, maxCol)
	}

	for r := 0; r < maxRow; r++ {
		// pacific ocean
		dfs(r, 0, reachPacific)
		// atlantic ocean
		dfs(r, maxCol-1, reachAtlantic)
	}

	for c := 0; c < maxCol; c++ {
		// pacific ocean
		dfs(0, c, reachPacific)
		// atlantic ocean
		dfs(maxRow-1, c, reachAtlantic)
	}

	// check
	for r := 0; r < maxRow; r++ {
		for c := 0; c < maxCol; c++ {
			if visited[r][c] == reachPacific+reachAtlantic {
				result = append(result, []int{r, c})
			}
		}
	}

	return result
}

func nextToOcean(r, c int, maxRow, maxCol int) uint8 {
	// handle 2 corners (reach both ocean)
	if r == 0 && c == maxCol-1 || r == maxRow-1 && c == 0 {
		return reachPacific + reachAtlantic
	}

	if r == 0 || c == 0 {
		return reachPacific
	}

	if r == maxRow-1 || c == maxCol-1 {
		return reachAtlantic
	}

	return 0
}
