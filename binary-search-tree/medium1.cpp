// https://leetcode.com/problems/check-completeness-of-a-binary-tree/
#include <algorithm>
#include <cassert>
#include <iostream>
#include <queue>
#include <stack>
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
};

bool isLeaf(TreeNode *node) { return node && !node->left && !node->right; }

class Solution {
public:
  bool isCompleteTree(TreeNode *root) {
    queue<TreeNode *> q;
    q.push(root);

    int levels = -1;
    bool no_more_allowed = false;
    while (!q.empty()) {
      levels++;
      cout << "levels: " << levels << '\n';
      int size = q.size();
      for (int i = 0; i < size; i++) {
        TreeNode *curr = q.front();
        if (curr->left) {
          if (no_more_allowed) {
            return false;
          }
          q.push(curr->left);
        } else if (!curr->left) {
          no_more_allowed = true;
        }
        if (curr->right) {
          if (no_more_allowed) {
            return false;
          }
          q.push(curr->right);
        } else if (!curr->right) {
          no_more_allowed = true;
        }
        cout << curr->val << " ";
        q.pop();
      }
      cout << '\n';
    }

    return true;
  }

  int depth(TreeNode *root) {
    if (!root)
      return 0;

    int l = depth(root->left);
    int r = depth(root->right);

    return max(l, r) + 1;
  }
  int diameterOfBinaryTree(TreeNode *root) {
    if (!root)
      return 0;

    int curr_dia = depth(root->left) + depth(root->right);

    int l_dia = diameterOfBinaryTree(root->left);
    int r_dia = diameterOfBinaryTree(root->right);

    return max(max(l_dia, r_dia), curr_dia);
  }

  bool flipEquiv(TreeNode *root1, TreeNode *root2) {
    queue<TreeNode *> q1;
    queue<TreeNode *> q2;

    q1.push(root1);
    q2.push(root2);

    while (!q1.empty()) {
      int size1 = q1.size();
      int size2 = q2.size();
      if (size1 != size2)
        return false;

      vector<int> v1;
      vector<int> v2;

      for (int i = 0; i < size1; i++) {
        TreeNode *cur1 = q1.front();
        TreeNode *cur2 = q2.front();

        q1.pop();
        q2.pop();

        if (cur1->left)
          q1.push(cur1->left), v1.push_back(cur1->left->val);

        if (cur2->left)
          q2.push(cur2->left), v2.push_back(cur2->left->val);

        if (cur1->right)
          q1.push(cur1->right), v1.push_back(cur1->right->val);

        if (cur2->right)
          q2.push(cur2->right), v2.push_back(cur2->right->val);
      }

      cout << "v1 vec\n";
      print(v1);

      cout << "v2 vec\n";
      print(v2);

      if (equal_vec(v1, v2) == false) {
        return false;
      }
    }
    return true;
  }

  bool equal_vec(vector<int> v1, vector<int> v2) {
    if (v1.size() != v2.size())
      return false;
    for (int i = 0; i < v1.size(); i++) {
      bool found = false;
      for (int j = 0; j < v2.size(); j++) {
        if (v1[i] == v2[j])
          v2[j] = -1, found = true;
      }
      if (!found)
        return false;
    }
    return true;
  }

  void print(vector<int> v) {
    for (int i = 0; i < v.size(); i++) {
      cout << v[i] << " ";
    }
    cout << '\n';
  }
};

int main() {

  BinaryTree tree(1);
  tree.add({2, 4, 7}, {'L', 'L', 'L'});
  tree.add({2, 4, 8}, {'L', 'L', 'R'});
  tree.add({2, 5, 9}, {'L', 'R', 'R'});
  tree.add({3, 6, 10}, {'R', 'R', 'L'});

  auto results = Solution().isCompleteTree(tree.root);

  cout << "\nbye\n";
  return 0;
}
