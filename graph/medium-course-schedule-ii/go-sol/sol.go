package gosol

func findOrder(numCourses int, prerequisites [][]int) []int {
	/*
		Idea: to learn a course, learn its prerequisties first
		Time: O(vertexs + edges) = O(numCourse + prerequisties) // visit each only once
		Space: O(n) // not include the adjList, 2 map to record the visiting and the visited
	*/

	// build the adj list
	var adjList = make(map[int][]int, len(prerequisites)) // vertex: course, edges: its prerequisties
	for i := range prerequisites {
		course := prerequisites[i][0]
		pre := prerequisites[i][1]
		adjList[course] = append(adjList[course], pre)
	}

	var (
		path     = make([]int, 0, numCourses) // result
		visiting = make(map[int]bool)
		visited  = make(map[int]bool, numCourses)
		dfs      func(course int) (canBeLearned bool)
	)
	dfs = func(course int) bool {
		if visited[course] {
			return true // no need to visit again
		}
		if visiting[course] {
			return false // found cyclic
		}

		// add to path
		visiting[course] = true

		// check all of its prerequisites
		for _, pre := range adjList[course] {
			if !dfs(pre) {
				return false // can not be learned
			}
		}
		// all of the course prerequisties is checked -> can learn this course
		path = append(path, course)
		visited[course] = true

		// remove from path
		visiting[course] = false

		return true
	}

	for i := 0; i < numCourses; i++ {
		if !dfs(i) {
			return nil
		}
	}

	return path
}
