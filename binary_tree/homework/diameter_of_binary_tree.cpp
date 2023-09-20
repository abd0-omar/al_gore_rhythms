
#include <algorithm>
#include <iostream>
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};
class Solution {
public:
  int depth(TreeNode *root) {
    if (!root) {
      return 0;
    }

    int l = depth(root->left);
    int r = depth(root->right);

    return std::max(l, r) + 1;
  }

  int diameterOfBinaryTree(TreeNode *root) {
    // if (!root->left && !root->right) {
    //   return 0;
    // }

    if (!root) {
      return 0;
    }

    int dia = depth(root->left) + depth(root->right);

    int l = diameterOfBinaryTree(root->left);
    int r = diameterOfBinaryTree(root->right);

    return std::max(dia, std::max(l, r));
    // return l + r;
    // int max =
    //     diameterOfBinaryTree(root->left) + diameterOfBinaryTree(root->right);
  }
};

int main() {
  // TreeNode *root = new TreeNode(5);
  // Solution solution;
  // int result = solution.diameterOfBinaryTree(root);
  // // Expected output: 0
  // std::cout << result << " ";

  TreeNode *root = new TreeNode(1);
  root->left = new TreeNode(2);
  root->right = new TreeNode(3);
  root->left->left = new TreeNode(4);
  root->left->right = new TreeNode(5);
  root->right->left = new TreeNode(6);
  root->right->right = new TreeNode(7);
  Solution solution;
  int result = solution.diameterOfBinaryTree(root);
  // Expected output: 4
  std::cout << result << " ";
}
