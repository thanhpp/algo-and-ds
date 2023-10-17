package gosol

/*
A course can not be completed if its prerequisites have a cycle
Use dfs and check if the path lead to a visited node
*/

func canFinish(numCourses int, prerequisites [][]int) bool {
	/*
		Time: O(course + len(prerequisites)) = O(vertexes + edges): visit each vertex and edge once
		Space: O(course) = O(vertexes): record path
	*/
	// create an adj from the prerequisites
	adjList := make(map[int][]int) // k -> v: course -> prerequisites
	for _, p := range prerequisites {
		adjList[p[0]] = append(adjList[p[0]], p[1])
	}

	// traverse using dfs
	var (
		dfs     func(course int) bool // is course can be finished
		visited = make(map[int]bool)  // path
	)
	dfs = func(course int) bool {
		if visited[course] { // found a cycle
			return false
		}

		if len(adjList[course]) == 0 {
			return true
		}

		visited[course] = true
		// check each prerequisite
		for _, p := range adjList[course] {
			if !dfs(p) {
				return false
			}
		}
		// not this path
		delete(visited, course)
		adjList[course] = nil // mark as can be finished (checked)

		return true
	}

	// for each course
	for i := 0; i < numCourses; i++ {
		if !dfs(i) {
			return false
		}
	}

	return true
}
