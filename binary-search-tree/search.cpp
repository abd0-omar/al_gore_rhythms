
struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

class Solution2 {
public:
  TreeNode *searchBST(TreeNode *root, int val) {
    if (!root) {
      return nullptr;
    }

    if (root->val == val) {
      return root;
    }

    if (val < root->val) {
      return searchBST(root->left, val);
    }

    return searchBST(root->right, val);
  }
};

class Solution {
public:
  TreeNode *lowestCommonAncestor(TreeNode *root, TreeNode *p, TreeNode *q) {
    if (!root) {
      return nullptr;
    }
    if (root->val > p->val && root->val > q->val) {
      return lowestCommonAncestor(root->left, p, q);
    }
    if (root->val < p->val && root->val < q->val) {
      return lowestCommonAncestor(root->right, p, q);
    }

    return root;
  }
};
