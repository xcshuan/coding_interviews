package find_duplicated

import "testing"

func TestFindDuplicated(t *testing.T) {
	a := [...]int{1, 3, 9, 6, 7, 6, 7, 8, 0, 2}
	res, ok := findDuplicatedNumbers1(a)
	if !ok {
		t.Error("failed")
	}
	switch res {
	case 6:
	case 7:
	default:
		t.Error("Fault", res)
	}
}

func isEqual(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := 0; i < len(a); i++ {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
