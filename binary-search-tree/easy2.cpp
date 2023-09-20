#include <vector>
using namespace std;
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
  TreeNode *searchBST(TreeNode *root, int val) {
    TreeNode *ok = root;
    bool found = false;
    dfs(root, val, found, ok);
    return ok;
  }

  // bool _search(TreeNode *root, int val) {
  //   bool found = false;
  //   TreeNode *ok = root;
  //   dfs(root, val, found, ok);
  //   return found;
  // }

  void dfs(TreeNode *root, int val, bool &found, TreeNode *&ok) {
    if (!root || found) {
      return;
    }

    if (root->val == val) {
      found = true;
      ok = root;
      return;
    }

    if (val < root->val) {
      dfs(root->left, val, found, ok);
    }

    if (val > root->val) {
      dfs(root->right, val, found, ok);
    }
  }
};

class Solution2 {
public:
  TreeNode *_builder(vector<int> &nums, int start, int end) {
    if (start > end) {
      return nullptr;
    }

    int mid = (start + end) / 2;

    TreeNode *root = new TreeNode(nums[mid]);
    root->left = _builder(nums, start, mid - 1);
    root->right = _builder(nums, mid + 1, end);
    return root;
  }

  TreeNode *sortedArrayToBST(vector<int> &nums) {
    return _builder(nums, 0, nums.size() - 1);
  }
};

class Solution3 {
public:
  bool _helper(TreeNode *root, TreeNode *left, TreeNode *right) {
    if (!root)
      return true;

    if ((left && root->val <= left->val) ||
        (right && root->val >= right->val)) {
      return false;
    }

    return _helper(root->left, left, root) && _helper(root->right, root, right);
  }

  bool isValidBST(TreeNode *root) { return _helper(root, nullptr, nullptr); }
};

class Solution4 {
public:
  int found = -1;
  int count = 0;

  void _helper(TreeNode *root, int k) {
    if (!root) {
      return;
    }

    _helper(root->left, k);
    if (++count == k) {
      found = root->val;
      return;
    }
    _helper(root->right, k);

    return;
  }

  int kthSmallest(TreeNode *root, int k) {
    _helper(root, k);
    return found;
  }
};
