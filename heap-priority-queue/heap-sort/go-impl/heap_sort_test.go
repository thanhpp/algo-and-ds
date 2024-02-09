package goimpl_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	goimpl "github.com/thanhpp/algo-and-ds/heap-priority-queue/heap-sort/go-impl"
	"golang.org/x/exp/slices"
)

func TestHeapSort(t *testing.T) {
	type TestCase struct {
		InputSlice []int
	}

	var testTable = []TestCase{
		{
			InputSlice: []int{1, 2, 3, 5, 2, 1, 23, 2, 5, 5, 6},
		},
	}

	for _, tc := range testTable {
		expect := slices.Clone(tc.InputSlice)
		slices.Sort(expect)

		t.Logf("\n%+v\n%+v", expect, tc.InputSlice)

		goimpl.HeapSort(tc.InputSlice)
		require.Equal(t, expect, tc.InputSlice)
	}
}
