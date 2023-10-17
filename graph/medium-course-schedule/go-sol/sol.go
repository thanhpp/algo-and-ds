package gosol

/*
Find every path, see if there any cyclic
*/

func canFinish1(numCourses int, prerequisites [][]int) bool {
	if numCourses == 0 {
		return true
	}
	if len(prerequisites) == 0 {
		return true
	}
	// build graph using a adj list
	var adjList = make(map[int][]int) // courses(values) are required to learn course[key]
	for _, pre := range prerequisites {
		adjList[pre[0]] = append(adjList[pre[0]], pre[1])
	}

	var (
		// cache to reduce time
		// if the current course required a checked course -> can return true
		canLearn = make(map[int]bool)
	)

	// dfs - to find a path
	var dfs func(visited map[int]bool, next int) bool
	dfs = func(visited map[int]bool, next int) bool {
		// learn all courses
		if len(visited) == numCourses {
			return true
		}

		// next course
		if visited[next] { // visted -> skip -> avoid cyclic
			return false
		}
		if canLearn[next] {
			return true
		}

		visited[next] = true
		pres := adjList[next]
		for _, p := range pres {
			// all pres are required -> can not continue
			if visited[p] {
				return false
			}
			if !dfs(visited, p) {
				return false
			}
		}
		delete(visited, next)

		return true
	}

	// try to go from each course number
	for i := 0; i < numCourses; i++ {
		var (
			visited = make(map[int]bool, numCourses)
		)
		if !dfs(visited, i) {
			return false
		}
		canLearn[i] = true
	}

	return true
}
