package gosol_test

import (
	"testing"

	gosol "github.com/thanhpp/algo-and-ds/dynamic-programming/2d-dp/medium-longest-common-subsequence/go-sol"
)

func TestSubsequence(t *testing.T) {
	var text = "abcde"
	sub := gosol.Subsequences(text)

	t.Log(sub)
}
