#include <algorithm>
#include <cassert>
#include <iostream>
#include <memory>
#include <string>
#include <vector>
using namespace std;

struct TreeNode { // don't copy for leetcode
  int val{};
  TreeNode *left{};
  TreeNode *right{};
  TreeNode(int val) : val(val) {}
};

struct BinaryTree {
  TreeNode *root{};
  BinaryTree(int root_value) : root(new TreeNode(root_value)) {}

  void add(vector<int> values, vector<char> direction) {
    assert(values.size() == direction.size());
    TreeNode *current = this->root;
    // iterate on the path, create all necessary TreeNodes
    for (int i = 0; i < (int)values.size(); ++i) {
      if (direction[i] == 'L') {
        if (!current->left)
          current->left = new TreeNode(values[i]);
        else
          assert(current->left->val == values[i]);
        current = current->left;
      } else {
        if (!current->right)
          current->right = new TreeNode(values[i]);
        else
          assert(current->right->val == values[i]);
        current = current->right;
      }
    }
  }

  void _printInorder(TreeNode *current) {
    if (!current)
      return;
    _printInorder(current->left);
    cout << current->val << " ";
    _printInorder(current->right);
  }

  void printInorder() {
    _printInorder(root);
    cout << "\n";
  }
};

bool isLeaf(TreeNode *node) { return node && !node->left && !node->right; }

class Solution {
public:
  void dfs(TreeNode *root, int &max) {
    if (!root)
      return;

    if (root->val > max)
      max = root->val;

    dfs(root->left, max);
    dfs(root->right, max);

    return;
  }
  int maxValue(TreeNode *root) {
    int max = root->val;
    dfs(root, max);
    return max;
  }

  void _max_depth(TreeNode *root, int count, int &max_count) {
    if (!root)
      return;

    if (count > max_count) {
      max_count = count;
    }
    _max_depth(root->left, count + 1, max_count);
    _max_depth(root->right, count + 1, max_count);

    return;
  }
  int max_depth(TreeNode *root) {
    int max_count = 0;
    _max_depth(root, 0, max_count);
    return max_count;
  }

  void _hasPathSum(TreeNode *root, int targetSum, int curr_sum, bool &found) {
    if (!root)
      return;

    curr_sum += root->val;

    if (!root->left && !root->right && targetSum == curr_sum) {
      found = true;
      return;
    }

    _hasPathSum(root->left, targetSum, curr_sum, found);
    _hasPathSum(root->right, targetSum, curr_sum, found);
  }
  bool hasPathSum(TreeNode *root, int targetSum) {
    bool found = false;
    _hasPathSum(root, targetSum, 0, found);
    return found;
  }

  void _sumOfLeftLeaves(TreeNode *root, bool is_left, int &count) {
    if (!root)
      return;

    if (!root->left && !root->right && is_left) {
      count += root->val;
    }

    _sumOfLeftLeaves(root->left, true, count);
    _sumOfLeftLeaves(root->right, false, count);
  }
  int sumOfLeftLeaves(TreeNode *root) {
    int count = 0;
    _sumOfLeftLeaves(root, false, count);
    return count;
  }

  // void _is_perfect(TreeNode *root) {}
  // bool is_perfect(TreeNode *root) {}
};

TreeNode *searchBST(TreeNode *root, int val) {
  if (!root)
    return nullptr;

  if (val == root->val) {
    return root;
  }

  if (val < root->val) {
    return searchBST(root->left, val);
  }

  return searchBST(root->right, val);
  /*
    while (root) {
      if (root->val == val) {
        return root;
      }

      if (root->val > val) {
root = root->left;
      }
      if (root->val < val) {
        root = root->right;
      }
    }
    return nullptr;
    */
}

// is_bst can be solved with min_max tree algo or with parenthesize canonical
// and normal parenthesize
// doesn't work, don't know why I thought it will work
string parenthesize(TreeNode *root) {
  if (!root)
    return "()";

  string str = "(" + to_string(root->val);

  if (root->left) {
    str += parenthesize(root->left);
  } else {
    str += "()";
  }

  if (root->right) {
    str += parenthesize(root->right);
  } else {
    str += "()";
  }

  str += ")";

  return str;
}

string parenthesize_canonical(TreeNode *root) {
  if (!root)
    return "()";

  string str = "(" + to_string(root->val);
  vector<string> v;

  if (root->left) {
    v.push_back(parenthesize(root->left));
  } else {
    v.push_back("()");
  }

  if (root->right) {
    v.push_back(parenthesize(root->right));
  } else {
    v.push_back("()");
  }

  sort(v.begin(), v.end());

  for (int i = 0; i < 2; i++) {
    str += v[i];
  }

  str += ")";

  return str;
}

bool is_bst(TreeNode *root) {
  string s1 = parenthesize(root);
  cout << "s1: " << s1 << "\n";

  string s2 = parenthesize_canonical(root);
  cout << "s2: " << s2 << "\n";

  return s1 == s2;
}

// TreeNode *sorted_to_bst(vector<TreeNode *> vec, TreeNode *root) {
// } // solved it on leetcode

int main() { // rename like main1 for leetcode
  BinaryTree tree(1);
  tree.add({2, 4, 7}, {'L', 'L', 'L'});
  tree.add({2, 4, 8}, {'L', 'L', 'R'});
  tree.add({2, 5, 9}, {'L', 'R', 'R'});
  tree.add({3, 6, 10}, {'R', 'R', 'L'});

  cout << Solution().hasPathSum(tree.root, 17) << "\n";
  cout << Solution().hasPathSum(tree.root, 170) << "\n";
  cout << Solution().hasPathSum(tree.root, 0) << "\n";
  cout << Solution().hasPathSum(tree.root, 7) << "\n";

  cout << "\n" << is_bst(tree.root);

  return 0;
}
