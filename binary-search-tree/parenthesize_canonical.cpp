//

#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <sstream> // ostringstream
#include <stack>
#include <vector>
using namespace std;

string toStr(int n) {
  // Convert integer to string
  ostringstream oss;
  oss << n;
  return oss.str();
}

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

  string parenthesize() { return _parenthesize(root); }

  string _parenthesize(TreeNode *root) {
    if (!root)
      return "()";

    string repr = "(" + toStr(root->val);

    if (root->left)
      repr += _parenthesize(root->left);
    else
      repr += "()"; // null: no child

    if (root->right)
      repr += _parenthesize(root->right);
    else
      repr += "()"; // null: no child
    repr += ")";

    return repr;
  }

  string parenthesize_canonical() { return _parenthesize_canonical(root); }

  string _parenthesize_canonical(TreeNode *root) {
    if (!root)
      return "()";

    string repr = "(" + toStr(root->val);

    vector<string> v;

    if (root->left) {
      v.push_back(_parenthesize_canonical(root->left));
    } else {
      v.push_back("()");
    }

    if (root->right) {
      v.push_back(_parenthesize_canonical(root->right));
    } else {
      v.push_back("()");
    }

    sort(v.begin(), v.end());
    for (int i = 0; i < v.size(); i++) {
      repr += v[i];
    }

    repr += ")";

    return repr;
  }

  // string _parenthesize_canonical(TreeNode *root) {
  // 	if (!root)
  // 		return "()";
  //
  // 	string repr = "(" + toStr(root->val);
  //
  // 	vector<string> v;
  //
  // 	if (left)
  // 		v.push_back(_parenthesize_canonical(root->left));
  // 	else
  // 		v.push_back("()");
  //
  // 	if (right)
  // 		v.push_back(_parenthesize_canonical(root->right));
  // 	else
  // 		v.push_back("()");
  //
  // 	sort(v.begin(), v.end());
  // 	for (int i = 0; i < (int) v.size(); ++i)
  // 		repr += v[i];
  //
  // 	repr += ")";
  //
  // 	return repr;
  // }
};

bool isLeaf(TreeNode *node) { return node && !node->left && !node->right; }

void test1() {
  BinaryTree tree(1);

  tree.add({3}, {'L'});
  tree.add({2}, {'R'});

  cout << tree.parenthesize() << "\n";
  cout << tree.parenthesize_canonical() << "\n";
}

void test2() {
  BinaryTree tree(1);
  tree.add({3, 8}, {'L', 'R'});
  tree.add({2}, {'R'});

  cout << tree.parenthesize() << "\n";
  cout << tree.parenthesize_canonical() << "\n";
}

void test3() {
  BinaryTree tree(1);

  tree.add({3, 4, 7}, {'L', 'L', 'L'});
  tree.add({3, 4, 8}, {'L', 'L', 'R'});
  tree.add({3, 5, 9}, {'L', 'R', 'R'});
  tree.add({3, 5, 13}, {'L', 'R', 'L'});

  tree.add({2, 6, 15}, {'R', 'R', 'L'});
  tree.add({2, 6, 12}, {'R', 'R', 'R'});
  tree.add({2, 14, 15}, {'R', 'L', 'L'});
  tree.add({2, 14, 16}, {'R', 'L', 'R'});

  cout << tree.parenthesize() << "\n";
  cout << tree.parenthesize_canonical() << "\n";
}

int main() {
  test1();
  test2();
  test3();

  cout << "\n\nbye\n";

  return 0;
}
