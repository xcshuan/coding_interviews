package nextNodeInBinaryTree

import "testing"

func TestNextNode(t *testing.T) {
	n7 := new(BinaryTreeNode)
	n7.value = 7
	n9 := new(BinaryTreeNode)
	n9.value = 9
	n4 := new(BinaryTreeNode)
	n4.value = 4
	n7.parent, n9.parent = n4, n4
	n4.left = n7
	n4.right = n9
	n2 := new(BinaryTreeNode)
	n2.value = 2
	n1 := new(BinaryTreeNode)
	n1.value = 1
	n4.parent, n2.parent = n1, n1
	n1.left = n4
	n1.right = n2
	a := nextNodeInBinaryTree(n7)
	if a.value != 4 {
		t.Error("falied")
	}
}
