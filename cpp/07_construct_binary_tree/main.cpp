#include <iostream>

class BinaryTreeNode {
public:
    int value;
    BinaryTreeNode *left;
    BinaryTreeNode *right;

    void print();
};

void BinaryTreeNode::print() {
    if (this == nullptr) {
        return;
    }
    if (this->left != nullptr) {
        this->left->print();
    }
    printf("%d\t", this->value);
    if (this->right != nullptr) {
        this->right->print();
    }
}

BinaryTreeNode *construct_core(int *start_preorder, int *end_preorder, int *start_inorder, int *end_inorder) {
    int root_value = start_preorder[0];
    BinaryTreeNode *root = new BinaryTreeNode();
    root->value = root_value;
    root->left = root->right = nullptr;

    if (start_preorder == end_preorder) {
        if (start_inorder == end_inorder && *start_inorder == *start_preorder) {
            return root;
        }
        throw std::exception();
    }

    int *root_inorder = nullptr;
    for (auto i = start_inorder; i <= end_inorder; i++) {
        if (*i == root_value) {
            root_inorder = i;
            break;
        }
    }
    if (root_inorder == nullptr) {
        throw std::exception();
    }
    int left_length = root_inorder - start_inorder;
    int *left_preorder_end = start_preorder + left_length;
    if (left_length > 0) {
        root->left = construct_core(start_preorder + 1, left_preorder_end, start_inorder, root_inorder - 1);
    }
    if (left_length < end_preorder - start_preorder) {
        root->right = construct_core(left_preorder_end + 1, end_preorder, root_inorder + 1, end_inorder);
    }
    return root;
}

BinaryTreeNode *construct_binary_tree(int *preorder, int *inorder, int length) {
    if (preorder == nullptr || inorder == nullptr || length <= 0) {
        return nullptr;
    }
    return construct_core(preorder, preorder + length - 1, inorder, inorder + length - 1);
}

int main() {
    int a[5] = {1, 4, 7, 9, 2};
    int b[5] = {7, 4, 9, 1, 2};
    auto tree = construct_binary_tree(a, b, 5);
    tree->print();
    return 0;
}