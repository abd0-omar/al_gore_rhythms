#include <iostream>
#include <vector>
using namespace std;

struct TreeNode {
  int val{};
  TreeNode *left{};
  TreeNode *right{};
  TreeNode(int val) : val(val) {}
};

class Solution {
public:
  TreeNode *bstFromPreorder(vector<int> &preorder) {
    int i = 0;
    return bstFromPreorder(preorder, i, 0, 1001);
  }
  TreeNode *bstFromPreorder(vector<int> &preorder, int i, int min, int max) {
    int data = preorder[i];

    TreeNode *root = new TreeNode(data);

    i += 1;

    if (preorder[i] < data && preorder[i] > min) {
      root->left = bstFromPreorder(preorder, i, min, data);
    }

    if (preorder[i] > data && preorder[i] < max) {
      root->left = bstFromPreorder(preorder, i, data, max);
    }

    return root;
  }

  void inorder(TreeNode *root) {
    if (!root)
      return;

    inorder(root->left);
    cout << root->val << " ";
    inorder(root->right);
  }
};

int main() {
  vector<int> v{20, 10, 30};

  Solution s;
  TreeNode *root = s.bstFromPreorder(v, 0, 0, 1001);
  s.inorder(root);
}
