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
  bool ok = true;

  void _helper(TreeNode *l, TreeNode *r) {
    if (!ok) {
      return; // Stop further recursive calls if ok is false
    }

    if (!l && !r) {
      ok = true;
      return; // Both subtrees are empty and symmetric
    }
    if (!l || !r) {
      ok = false;
      return;
    }
    if (l->val != r->val) {
      ok = false;
      return;
    }
    _helper(l->left, r->right);
    _helper(l->right, r->left);
  }

  bool isSymmetric(TreeNode *root) {
    _helper(root->left, root->right);
    return ok;
  }
};
