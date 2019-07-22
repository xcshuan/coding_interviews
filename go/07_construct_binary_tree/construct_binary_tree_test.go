package constructbinarytree

import (
	"fmt"
	"testing"
)

func TestBinaryTree(t *testing.T) {
	// a := []int{7, 4, 9, 1, 2}
	// b := []int{1, 4, 7, 9, 2}
	a := []int{1, 2}
	b := []int{2, 1}
	tree, _ := construcBinaryTree(a, b)
	tree.print()
	fmt.Println("")
	t.Error(" ")
}
