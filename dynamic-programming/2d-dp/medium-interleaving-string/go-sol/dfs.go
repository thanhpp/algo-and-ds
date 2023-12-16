package gosol

/*
TIME COMPLEXITY: 2^n
- each position: there are 2 options (take from s1 or take from s2)
*/
func isInterleaveDFS(s1 string, s2 string, s3 string) bool {
	if len(s3) == 0 {
		return true
	}
	if len(s1)+len(s2) != len(s3) {
		return false
	}
	return properDFS(s1, s2, s3, 0, 0, 0)
}

func properDFS(s1, s2, s3 string, i1, i2, i3 int) bool {
	if i3 == len(s3) {
		return true
	}

	c3 := s3[i3]
	v1, v2 := false, false
	if i1 < len(s1) {
		c1 := s1[i1]
		if c1 == c3 {
			v1 = properDFS(s1, s2, s3, i1+1, i2, i3+1) // continue to take from s1
		}
	}
	if i2 < len(s2) {
		c2 := s2[i2]
		if c2 == c3 {
			v2 = properDFS(s1, s2, s3, i1, i2+1, i3+1)
		}
	}

	return v1 || v2
}
