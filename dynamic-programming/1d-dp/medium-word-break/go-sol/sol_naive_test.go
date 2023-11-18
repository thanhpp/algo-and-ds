package gosol_test

import (
	"testing"

	"github.com/stretchr/testify/assert"
	gosol "github.com/thanhpp/algo-and-ds/dynamic-programming/1d-dp/medium-word-break/go-sol"
)

func TestNaive(t *testing.T) {
	var (
		s        = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"
		wordDict = []string{"a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"}
		expected = false
	)

	assert.Equal(t, gosol.WordBreakNaive(s, wordDict), expected)
}
