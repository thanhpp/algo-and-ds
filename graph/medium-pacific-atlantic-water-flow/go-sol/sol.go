package gosol

// naive solution
// Big(O) = O((m*n)^2) --> each node has to re-visit all of the matrix
func pacificAtlanticNaive(heights [][]int) [][]int {
	var visited = make([][]bool, len(heights))
	var result [][]int
	for i := 0; i < len(heights); i++ {
		visited[i] = make([]bool, len(heights[i]))
	}

	for r := 0; r < len(heights); r++ {
		for c := 0; c < len(heights[0]); c++ {
			resetVisited(visited)
			isPacific, isAtlantic := flow(r, c, heights, visited)
			if isPacific && isAtlantic {
				result = append(result, []int{r, c})
			}
		}
	}

	return result
}

func flow(r, c int, heights [][]int, visited [][]bool) (isPacific bool, isAtlantic bool) {
	currentPacific, currentAtlantic := pacific(r, c), atlantic(r, c, len(heights), len(heights[0]))
	visited[r][c] = true

	if !(currentPacific && currentAtlantic) && r > 0 && !visited[r-1][c] && heights[r][c] >= heights[r-1][c] {
		isPacific, isAtlantic := flow(r-1, c, heights, visited)
		currentPacific = currentPacific || isPacific
		currentAtlantic = currentAtlantic || isAtlantic
	}
	if !(currentPacific && currentAtlantic) && r < len(heights)-1 && !visited[r+1][c] && heights[r][c] >= heights[r+1][c] {
		isPacific, isAtlantic := flow(r+1, c, heights, visited)
		currentPacific = currentPacific || isPacific
		currentAtlantic = currentAtlantic || isAtlantic
	}
	if !(currentPacific && currentAtlantic) && c > 0 && !visited[r][c-1] && heights[r][c] >= heights[r][c-1] {
		isPacific, isAtlantic := flow(r, c-1, heights, visited)
		currentPacific = currentPacific || isPacific
		currentAtlantic = currentAtlantic || isAtlantic
	}
	if !(currentPacific && currentAtlantic) && c < len(heights[0])-1 && !visited[r][c+1] && heights[r][c] >= heights[r][c+1] {
		isPacific, isAtlantic := flow(r, c+1, heights, visited)
		currentPacific = currentPacific || isPacific
		currentAtlantic = currentAtlantic || isAtlantic
	}

	return currentPacific, currentAtlantic
}

func pacific(r, c int) bool {
	return r == 0 || c == 0
}

func atlantic(r, c int, rMax, cMax int) bool {
	return r == rMax-1 || c == cMax-1
}

func resetVisited(visited [][]bool) {
	for r := 0; r < len(visited); r++ {
		for c := 0; c < len(visited[0]); c++ {
			visited[r][c] = false
		}
	}
}
