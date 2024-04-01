package gosol

import "testing"

func TestPartitionLabels(t *testing.T) {
	s := "ababcbacadefegdehijhklij"
	t.Log(partitionLabels(s))
}
