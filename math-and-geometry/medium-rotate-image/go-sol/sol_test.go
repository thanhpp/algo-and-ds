package gosol

import (
	"fmt"
	"testing"
)

func TestRotate1(t *testing.T) {
	m := [][]int{
		{1, 2},
		{4, 3},
	}
	rotate(m)

	for i := range m {
		fmt.Println(m[i])
	}
}

func TestRotate2(t *testing.T) {
	m := [][]int{
		{1, 2, 3},
		{8, 9, 4},
		{7, 6, 5},
	}
	rotate(m)

	for i := range m {
		fmt.Println(m[i])
	}
}

func TestRotate3(t *testing.T) {
	m := [][]int{
		{1, 2, 3},
		{4, 5, 6},
		{7, 8, 9},
	}
	rotate(m)

	for i := range m {
		fmt.Println(m[i])
	}
}

func TestRotate4(t *testing.T) {
	m := [][]int{
		{5, 1, 9, 11},
		{2, 4, 8, 10},
		{13, 3, 6, 7},
		{15, 14, 12, 16},
	}
	rotate(m)

	for i := range m {
		fmt.Println(m[i])
	}
}
