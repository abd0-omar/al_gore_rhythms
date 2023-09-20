#include <utility>
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

void find_chain(TreeNode *root, vector<TreeNode *> &ancestors, int val,
                bool found) {
  if (!root || found)
    return;

  if (root->val == val) {
    ancestors.push_back(root);
    found = true;
    return;
  }

  if (val < root->val) {
    find_chain(root->left, ancestors, val, found);
  }

  ancestors.push_back(root);

  if (val > root->val) {
    find_chain(root->right, ancestors, val, found);
  }
}

bool _search(TreeNode *root, int target) {
  if (!root)
    return false;

  if (target == root->val)
    return true;

  if (target < root->val) {
    return _search(root->left, target);
  }

  return _search(root->right, target);
}

// void _insert(TreeNode *root, TreeNode *target) {
//   if (!root) {
//     root = target;
//   }
//
//   if (target->val < root->val) {
//     _insert(root->left, target);
//   }
//
//   if (target->val > root->val) {
//     _insert(root->right, target);
//   }
// }

void _insert(TreeNode *&root, int target) {
  if (!root) {
    root = new TreeNode(target);
    return;
  }

  if (target < root->val) {
    if (!root->left)
      root->left = new TreeNode(target);
    else
      _insert(root->left, target);
  } else if (target > root->val) {
    if (!root->right)
      root->right = new TreeNode(target);
    else
      _insert(root->right, target);
  }
}

TreeNode *min_node(TreeNode *cur) {
  while (cur && cur->left) {
    cur = cur->left;
  }
  return cur;
}

pair<TreeNode *, TreeNode *> tree_to_dll(TreeNode *root) {
  pair<TreeNode *, TreeNode *> left = tree_to_dll(root->left);
  pair<TreeNode *, TreeNode *> right = tree_to_dll(root->right);

  pair<TreeNode *, TreeNode *> roots = make_pair(root, root);

  if (left.first) {
    left.second->right = roots.second;
    roots.first->left = left.second;
  }
  if (right.first) {
    right.first->left = roots.second;
    roots.second->right = right.first;
  }

  return make_pair(left.first, right.second);
  // fails if left doesn't exist or right doesn't exist,
  // needs some if statements but too lazy to do them tbh.
}
