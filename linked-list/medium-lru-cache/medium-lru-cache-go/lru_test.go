package lru_test

import (
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	lru "github.com/thanhpp/algo-and-ds/linked-list/medium-lru-cache/medium-lru-cache-go"
)

func TestLRU(t *testing.T) {
	cache := lru.Constructor(2)

	cache.Put(1, 1)

	cache.Put(2, 2)

	assert.Equal(t, 1, cache.Get(1))

	cache.Put(3, 3)

	assert.Equal(t, -1, cache.Get(2))

	cache.Put(4, 4)

	assert.Equal(t, -1, cache.Get(1))

	assert.Equal(t, 3, cache.Get(3))

	assert.Equal(t, 4, cache.Get(4))
}

// ["LRUCache",	"put",	"put",	"get",	"put",	"get",	"put",	"get",	"get",	"get"]
// [[2],		[1,0],	[2,2],	[1],	[3,3],	[2],	[4,4],	[1],	[3],	[4]]
// [null,		null,	null,	0,		null,	-1,		null,	-1,		3,		4]
func TestLRU2(t *testing.T) {
	cache := lru.Constructor(2)

	cache.Put(1, 0)

	cache.Put(2, 2)

	assert.Equal(t, 0, cache.Get(1))

	cache.Put(3, 3)

	assert.Equal(t, -1, cache.Get(2))

	cache.Put(4, 4)

	assert.Equal(t, -1, cache.Get(1))

	assert.Equal(t, 3, cache.Get(3))

	assert.Equal(t, 4, cache.Get(4))
}

func TestLRU3(t *testing.T) {
	// ["LRUCache",	"put",	"put",	"put",	"put",	"get",	"get",	"get",	"get",	"put",	"get",	"get",	"get",	"get",	"get"]
	// [[3],		[1,1],	[2,2],	[3,3],	[4,4],	[4],	[3],	[2],	[1],	[5,5],	[1],	[2],	[3],	[4],	[5]]
	// [null,		null,	null,	null,	null,	4,		3,		2,		-1,		null,	-1,		2,		3,		-1,		5]

	cache := lru.Constructor(3)
	cache.Put(1, 1)
	cache.Put(2, 2)
	cache.Put(3, 3)
	cache.Put(4, 4)

	assert.Equal(t, 4, cache.Get(4))

	assert.Equal(t, 3, cache.Get(3))

	assert.Equal(t, 2, cache.Get(2))

	assert.Equal(t, -1, cache.Get(1))

	cache.Put(5, 5)

	assert.Equal(t, -1, cache.Get(1))

	assert.Equal(t, 2, cache.Get(2))

	assert.Equal(t, 3, cache.Get(3))

	assert.Equal(t, -1, cache.Get(4))

	assert.Equal(t, 5, cache.Get(5))
}

func TestLRU4(t *testing.T) {
	test(
		t,
		[]string{"LRUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get", "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put"},
		[][]int{{10}, {10, 13}, {3, 17}, {6, 11}, {10, 5}, {9, 10}, {13}, {2, 19}, {2}, {3}, {5, 25}, {8}, {9, 22}, {5, 5}, {1, 30}, {11}, {9, 12}, {7}, {5}, {8}, {9}, {4, 30}, {9, 3}, {9}, {10}, {10}, {6, 14}, {3, 1}, {3}, {10, 11}, {8}, {2, 14}, {1}, {5}, {4}, {11, 4}, {12, 24}, {5, 18}, {13}, {7, 23}, {8}, {12}, {3, 27}, {2, 12}, {5}, {2, 9}, {13, 4}, {8, 18}, {1, 7}, {6}, {9, 29}, {8, 21}, {5}, {6, 30}, {1, 12}, {10}, {4, 15}, {7, 22}, {11, 26}, {8, 17}, {9, 29}, {5}, {3, 4}, {11, 30}, {12}, {4, 29}, {3}, {9}, {6}, {3, 4}, {1}, {10}, {3, 29}, {10, 28}, {1, 20}, {11, 13}, {3}, {3, 12}, {3, 8}, {10, 9}, {3, 26}, {8}, {7}, {5}, {13, 17}, {2, 27}, {11, 15}, {12}, {9, 19}, {2, 15}, {3, 16}, {1}, {12, 17}, {9, 1}, {6, 19}, {4}, {5}, {5}, {8, 1}, {11, 7}, {5, 2}, {9, 28}, {1}, {2, 2}, {7, 4}, {4, 22}, {7, 24}, {9, 26}, {13, 28}, {11, 26}},
		[]string{"null", "null", "null", "null", "null", "null", "-1", "null", "19", "17", "null", "-1", "null", "null", "null", "-1", "null", "-1", "5", "-1", "12", "null", "null", "3", "5", "5", "null", "null", "1", "null", "-1", "null", "30", "5", "30", "null", "null", "null", "-1", "null", "-1", "24", "null", "null", "18", "null", "null", "null", "null", "-1", "null", "null", "18", "null", "null", "-1", "null", "null", "null", "null", "null", "18", "null", "null", "-1", "null", "4", "29", "30", "null", "12", "-1", "null", "null", "null", "null", "29", "null", "null", "null", "null", "17", "22", "18", "null", "null", "null", "-1", "null", "null", "null", "20", "null", "null", "null", "-1", "18", "18", "null", "null", "null", "null", "20", "null", "null", "null", "null", "null", "null", "null"},
	)
}

func test(t *testing.T, in1 []string, in2 [][]int, expected []string) {
	var cache lru.LRUCache
	for i := range in1 {
		switch in1[i] {
		case "LRUCache":
			cache = lru.Constructor(in2[i][0])
		case "put":
			cache.Put(in2[i][0], in2[i][1])
		case "get":
			expect, _ := strconv.Atoi(expected[i])
			require.Equal(t, expect, cache.Get(in2[i][0]))
		}
	}
}
