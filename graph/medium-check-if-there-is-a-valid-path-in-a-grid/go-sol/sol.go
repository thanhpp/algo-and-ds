package gosol

func hasValidPath(grid [][]int) bool {
	visited := make(map[[3]int]bool)

	switch grid[0][0] {
	case 1:
		return dfs(grid, 0, 0, DirectionRight, visited)

	case 2:
		return dfs(grid, 0, 0, DirectionDown, visited)

	case 3:
		v1 := dfs(grid, 0, 0, DirectionRight, visited)
		if v1 {
			return true
		}
		clearMap(visited)
		return dfs(grid, 0, 0, DirectionUp, visited)

	case 4:
		v1 := dfs(grid, 0, 0, DirectionLeft, visited)
		if v1 {
			return true
		}
		clearMap(visited)
		return dfs(grid, 0, 0, DirectionUp, visited)

	case 5:
		v1 := dfs(grid, 0, 0, DirectionDown, visited)
		if v1 {
			return true
		}
		clearMap(visited)
		return dfs(grid, 0, 0, DirectionRight, visited)

	case 6:
		v1 := dfs(grid, 0, 0, DirectionDown, visited)
		if v1 {
			return true
		}
		clearMap(visited)
		return dfs(grid, 0, 0, DirectionLeft, visited)
	}

	return false
}

func dfs(grid [][]int, x, y int, direction Direction, visited map[[3]int]bool) bool {
	// fmt.Printf("x: %d | y: %d | dir: %d\n", x, y, direction)

	if (x == len(grid)-1) && (y == len(grid[0])-1) {
		_, _, dir := next(x, y, grid[x][y], direction)
		return dir != 0
	}
	if x < 0 || y < 0 {
		return false
	}
	if x >= len(grid) || y >= len(grid[0]) {
		return false
	}

	if visited[[3]int{x, y, int(direction)}] {
		return false
	}

	nextX, nextY, dir := next(x, y, grid[x][y], direction)
	if dir == 0 {
		return false
	}
	visited[[3]int{x, y, int(direction)}] = true

	return dfs(grid, nextX, nextY, dir, visited)
}

func next(x, y, val int, prevDirection Direction) (int, int, Direction) {
	// fmt.Printf("next x: %d | y: %d | val: %d | dir: %d\n", x, y, val, prevDirection)

	switch prevDirection {
	case DirectionUp:
		switch val {
		case 1, 5, 6:
			return -1, -1, 0
		case 2:
			return x - 1, y, DirectionUp
		case 3:
			return x, y - 1, DirectionLeft
		case 4:
			return x, y + 1, DirectionRight
		}

	case DirectionDown:
		switch val {
		case 1, 3, 4:
			return -1, -1, 0
		case 2:
			return x + 1, y, DirectionDown
		case 5:
			return x, y - 1, DirectionLeft
		case 6:
			return x, y + 1, DirectionRight
		}

	case DirectionLeft:
		switch val {
		case 2, 3, 5:
			return -1, -1, 0

		case 1:
			return x, y - 1, DirectionLeft

		case 4:
			return x + 1, y, DirectionDown

		case 6:
			return x - 1, y, DirectionUp
		}

	case DirectionRight:
		switch val {
		case 2, 4, 6:
			return -1, -1, 0

		case 1:
			return x, y + 1, DirectionRight

		case 3:
			return x + 1, y, DirectionDown

		case 5:
			return x - 1, y, DirectionUp
		}
	}

	panic("invalid")
}

type Direction int

const (
	DirectionUp Direction = iota + 1
	DirectionDown
	DirectionLeft
	DirectionRight
)

func clearMap(m map[[3]int]bool) {
	for k := range m {
		delete(m, k)
	}
}
