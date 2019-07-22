package constructbinarytree

import (
	"errors"
	"fmt"
)

type BinaryTreeNode struct {
	value int
	left  *BinaryTreeNode
	right *BinaryTreeNode
}

//中序打印
func (t *BinaryTreeNode) print() {
	if t == nil {
		fmt.Println("Tree is nil")
		return
	}
	var temp = t
	if t.left != nil {
		t.left.print()
	}
	if temp != nil {
		fmt.Printf("%d\t", temp.value)
	}
	if t.right != nil {
		t.right.print()
	}
}

func construcBinaryTree(preorder, inorder []int) (*BinaryTreeNode, error) {
	if (len(preorder) != len(inorder)) || preorder == nil {
		return nil, errors.New("failed")
	}
	rootValue := preorder[0]
	n := new(BinaryTreeNode)
	n.value = rootValue
	var err error
	if len(preorder) == 1 {
		if preorder[0] == inorder[0] {
			return n, nil
		}
		return nil, errors.New("failed")
	}

	if rootValue == inorder[len(inorder)-1] {
		n.left, err = construcBinaryTree(preorder[1:], inorder[:len(inorder)-1])
		if err != nil {
			return nil, err
		}
	} else if rootValue == inorder[0] {
		n.right, err = construcBinaryTree(preorder[1:], inorder[1:])
		if err != nil {
			return nil, err
		}
	} else {
		for i, j := range inorder {
			if j == rootValue {
				n.left, err = construcBinaryTree(preorder[1:i+1], inorder[0:i])
				if err != nil {
					return nil, err
				}
				n.right, err = construcBinaryTree(preorder[i+1:], inorder[i+1:])
				if err != nil {
					return nil, err
				}
				break
			} else if i == len(inorder)-1 {
				return nil, errors.New("failed")
			}
		}
	}
	return n, nil
}
