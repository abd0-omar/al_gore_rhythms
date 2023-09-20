#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

struct TreeNode {
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

  void _print_inorder(TreeNode *current) {
    if (!current)
      return;
    _print_inorder(current->left);
    cout << current->val << " ";
    _print_inorder(current->right);
  }

  void print_inorder() {
    _print_inorder(root);
    cout << "\n";
  }

  bool _search(TreeNode *root, int target) { // tail recursion
    if (!root)
      return false;

    if (target == root->val)
      return true;

    if (target < root->val)
      return _search(root->left, target);

    return _search(root->right, target);
  }

  bool search(int target) { return _search(root, target); }
};

int main() {
  // 					50
  //		30							70
  //	10		40				60
  // 80
  //     12                55                        90

  BinaryTree tree(50);
  tree.add({30, 10, 12}, {'L', 'L', 'R'});
  tree.add({30, 40}, {'L', 'R'});
  tree.add({70, 60, 55}, {'R', 'L', 'L'});
  tree.add({70, 80, 90}, {'R', 'R', 'R'});

  tree.print_inorder();

  assert(tree.search(12) == true);
  assert(tree.search(90) == true);

  // Not a  BST now
  tree.add({70, 80, 100}, {'R', 'R', 'L'});
  assert(tree.search(100) == false);

  return 0;
}
