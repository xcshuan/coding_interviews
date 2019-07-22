package printlistreverse

import (
	"container/list"
	"fmt"
)

type ListNode struct {
	value int
	next  *ListNode
}

func printListReversingly(head *ListNode) {
	if head == nil {
		return
	}
	if head.next == nil {
		fmt.Printf("%d", head.value)
	} else {
		printListReversingly(head.next)
		fmt.Printf("<-%d", head.value)
	}
}

func printListReversingly2(head *ListNode) {
	if head == nil {
		return
	}
	ls := list.New()
	var n1 = head
	for n1 != nil {
		ls.PushBack(n1)
		n1 = n1.next
	}
	n2 := ls.Back()
	for n2 != nil {
		fmt.Printf("%d", n2.Value.(*ListNode).value)
		if n2.Prev() != nil {
			fmt.Printf("->")
		}
		n2 = n2.Prev()
	}
}
