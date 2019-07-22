package nextNodeInBinaryTree

type BinaryTreeNode struct {
	value  int
	parent *BinaryTreeNode
	left   *BinaryTreeNode
	right  *BinaryTreeNode
}

func nextNodeInBinaryTree(node *BinaryTreeNode) *BinaryTreeNode {
	if node == nil {
		return nil
	}
	var temp *BinaryTreeNode
	if node.right != nil {
		temp = node.right
		for temp.left != nil {
			temp = temp.left
		}
		return temp
	} else if node.parent != nil && node.parent.left == node {
		return node.parent
	} else if node.parent != nil {
		temp = node.parent
		for temp.parent != nil {
			if temp.parent.left == temp.parent {
				return temp.parent
			}
			temp = temp.parent
		}
	}
	return nil
}
