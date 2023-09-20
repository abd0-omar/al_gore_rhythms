#include <algorithm>
#include <assert.h>
#include <iostream>
#include <vector>
using namespace std;

struct TreeNode {
  int val{};
  TreeNode *left{};
  TreeNode *right{};
  TreeNode(int data) : val(data) {}
};

struct BinaryTree {
  TreeNode *root{};

  BinaryTree(int root_value) : root(new TreeNode(root_value)) {}

  void _print_inorder(TreeNode *current) {
    if (!current) {
      return;
    }

    _print_inorder(current->left);
    cout << current->val << " ";
    _print_inorder(current->right);
  }

  void print_inorder() {
    _print_inorder(root);
    return;
  }

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

  int _tree_max(TreeNode *current) {
    if (!current) {
      return 0;
    }
    int max = current->val;
    max = std::max(max, _tree_max(current->left));
    max = std::max(max, _tree_max(current->right));
    return max;
  }

  int tree_max() { return _tree_max(root); }

  int _maximum_depth(TreeNode *current) {
    if (!current) {
      return 0;
    }

    int max = 1;

    if (current->left) {
      max = std::max(max, _maximum_depth(current->left) + 1);
    }
    if (current->right) {
      max = std::max(max, _maximum_depth(current->right) + 1);
    }

    return max;
  }

  int _sum(TreeNode *current) {
    if (!current) {
      return 0;
    }

    int sum = current->val;

    sum = sum + _sum(current->left);
    sum = sum + _sum(current->right);

    return sum;
  }

  int sum() { return _sum(root); }

  bool _hasPathSum_left_or_right(TreeNode *current, int targetSum) {
    // if (current->val == targetSum)
    //   return true;
    // else
    //   return false;

    if (!current) {
      return false;
    }

    // if (_sum(current->left) == targetSum) {
    //   return true;
    // }
    //
    // if (_sum(current->right) == targetSum) {
    //   return true;
    // }

    if (_sum(current) == targetSum) {
      return true;
    }

    return _hasPathSum_left_or_right(current->left, targetSum) ||
           _hasPathSum_left_or_right(current->right, targetSum);

    // int suml = _hasPathSum(current->left, targetSum);
    // int sumr = _hasPathSum(current->right, targetSum);

    // if (current->left) {
    //   return _sum(current->left) == targetSum;
    // }
    // if (current->right) {
    //   return _sum(current->right) == targetSum;
    // }
    //
    // return _hasPathSum(current->left, int targetSum)
  }

  bool _hasPathSum(TreeNode *current, int targetSum, int currSum = 0) {

    if (!current) {
      return false;
    }

    currSum += current->val;

    if (targetSum == currSum && !current->left && !current->right) {
      return true;
    }

    return _hasPathSum(current->left, targetSum, currSum) ||
           _hasPathSum(current->right, targetSum, currSum);
  }

  bool hasPathSum(int targetSum) { return _hasPathSum(root, targetSum, 0); }

  bool hasPathSum_left_or_right(int targetSum) {
    return _hasPathSum_left_or_right(root, targetSum);
  }

  int maximum_depth() { return _maximum_depth(root); }

  int sumOfLeftLeaves(TreeNode *root, bool isLeft = false) {
    if (!root) {
      return false;
    }

    int sum = 0;

    if (!root->left && !root->right) {
      if (isLeft) {
        sum += root->val;
      }
    }

    sum += sumOfLeftLeaves(root->left, true);
    sum += sumOfLeftLeaves(root->right, false);

    return sum;
  }
};

int main() {
  BinaryTree tree(1);
  tree.add({2, 4, 7}, {'L', 'L', 'L'});
  tree.add({2, 4, 8}, {'L', 'L', 'R'});
  tree.add({2, 5, 9}, {'L', 'R', 'R'});
  tree.add({3, 6, 10}, {'R', 'R', 'L'});

  tree.print_inorder();
  cout << '\n';
  // 7 4 8 2 5 9 1 3 10 6

  cout << tree.tree_max();
  cout << '\n';
  // 10

  cout << tree.maximum_depth();
  cout << '\n';

  cout << tree.sum();
  cout << '\n';

  cout << tree.hasPathSum(50);
  cout << '\n';

  return 0;
}
