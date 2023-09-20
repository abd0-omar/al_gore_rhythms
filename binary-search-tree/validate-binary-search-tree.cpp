
#include <climits>
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
  bool dfs(TreeNode *root, long long min = LONG_LONG_MIN,
           long long max = LONG_LONG_MAX) {

    bool status = root->val > min && root->val < max;

    if (!status) {
      return false;
    }

    bool left = !root->left || dfs(root->left, min, root->val);

    if (!left) {
      return false;
    }

    bool right = !root->right || dfs(root->right, root->val, max);
    return right;
  }
  bool isValidBST(TreeNode *root) {
    if (!root) {
      return true;
    }

    return dfs(root);
  }
};
