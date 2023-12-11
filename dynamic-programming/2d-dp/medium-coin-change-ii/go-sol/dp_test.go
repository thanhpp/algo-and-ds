package gosol_test

import (
	"testing"

	gosol "github.com/thanhpp/algo-and-ds/dynamic-programming/2d-dp/medium-coin-change-ii/go-sol"
)

func TestDPChange(t *testing.T) {
	r := gosol.Change(5, []int{1, 2, 5})
	t.Log(r)
}
