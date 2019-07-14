package find_in_2dmatrix

import "testing"

func TestFindIn2DMatrix(t *testing.T) {
	a := [][]int{{1, 2, 8, 9}, {2, 4, 9, 12}, {4, 7, 10, 13}, {6, 8, 11, 15}}
	if !findIn2DMatrix(a, 4, 4, 7) {
		t.Error("Failed")
	}
}
