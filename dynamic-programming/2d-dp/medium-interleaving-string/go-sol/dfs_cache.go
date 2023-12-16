package gosol

import "fmt"

/* TIME COMPLEXITY: s1 * s2 (Calculate all posible states)
- each position: possible states len(s1) * len(s2) => SPACE O(s1 * s2)
*/

// KEY: interleaving => i3 = i1 + i2

func isInterleaveDFSCache(s1 string, s2 string, s3 string) bool {
	if len(s3) == 0 {
		return true
	}
	if len(s1)+len(s2) != len(s3) {
		return false
	}
	return dfsCache(s1, s2, s3, 0, 0, 0, make(map[string]bool))
}

func dfsCache(s1, s2, s3 string, i1, i2, i3 int, cache map[string]bool) bool {
	if i3 == len(s3) {
		return true
	}
	key := fmt.Sprintf("%d-%d", i2, i1) // CACHE SIZE = i1 * i2
	if b, ok := cache[key]; ok {
		return b
	}

	c3 := s3[i3]
	v1, v2 := false, false
	if i1 < len(s1) {
		c1 := s1[i1]
		if c1 == c3 {
			v1 = dfsCache(s1, s2, s3, i1+1, i2, i3+1, cache) // continue to take from s1
		}
	}
	if i2 < len(s2) {
		c2 := s2[i2]
		if c2 == c3 {
			v2 = dfsCache(s1, s2, s3, i1, i2+1, i3+1, cache)
		}
	}

	cache[key] = v1 || v2

	return v1 || v2
}
