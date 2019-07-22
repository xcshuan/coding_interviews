#include <iostream>

class BinaryTreeNode {
public:
    int value;
    BinaryTreeNode
            *parent;
    BinaryTreeNode *left;
    BinaryTreeNode
            *right;
};

BinaryTreeNode *next_node_in_binary_tree(BinaryTreeNode *node) {
    if (node == nullptr) {
        return nullptr;
    }
    BinaryTreeNode *temp;
    if (node->right != nullptr) {
        temp = node->right;
        while (temp->left != nullptr) {
            temp = temp->left;
        }
        return temp;
    } else if (node->parent != nullptr && node->parent->left == node) {
        return node->parent;
    } else if (node->parent != nullptr) {
        temp = node->parent;
        while (temp->parent != nullptr) {
            if (temp->parent->left == temp->parent) {
                return temp->parent;
            }
            temp = temp->parent;
        }
    }
    return nullptr;
}

int main() {
    auto n7 = new(BinaryTreeNode);
    n7->value = 7;
    auto n9 = new(BinaryTreeNode);
    n9->value = 9;
    auto n4 = new(BinaryTreeNode);
    n4->value = 4;
    n7->parent = n4;
    n9->parent = n4;
    n4->left = n7;
    n4->right = n9;
    auto n2 = new(BinaryTreeNode);
    n2->value = 2;
    auto n1 = new(BinaryTreeNode);
    n1->value = 1;
    n4->parent = n1;
    n2->parent = n1;
    n1->left = n4;
    n1->right = n2;
    auto a = next_node_in_binary_tree(n7);
    if (a != n4) {
        std::cout << "Failed" << std::endl;
    }
    std::cout << "Hello, World!" << std::endl;
    return 0;
}