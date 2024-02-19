package gosol

import (
	"fmt"
	"testing"
)

func TestLeastInterval(t *testing.T) {
	fmt.Println(leastInterval([]byte{'A', 'A', 'A', 'B', 'B', 'B'}, 2))
}
