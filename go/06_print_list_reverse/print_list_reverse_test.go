package printlistreverse

import "testing"

func TestPrintList(t *testing.T) {
	a := ListNode{
		3,
		&ListNode{
			4,
			&ListNode{5, nil},
		},
	}
	printListReversingly2(&a)
}
