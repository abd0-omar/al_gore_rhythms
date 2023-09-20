#include <climits>
#include <deque>
#include <iostream>
#include <queue>
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

TreeNode *buildTree(const std::vector<int> &values, int index) {
  if (index >= values.size() || values[index] == INT_MAX) {
    return nullptr;
  }

  TreeNode *newNode = new TreeNode(values[index]);
  newNode->left = buildTree(values, 2 * index + 1);
  newNode->right = buildTree(values, 2 * index + 2);

  return newNode;
}

void print(TreeNode *root) {
  queue<TreeNode *> q;
  q.push(root);
  int size = q.size();
  // while (size--) {
  while (!q.empty()) {

    TreeNode *cur = q.front();

    if (cur->left) {
      q.push(cur->left);
    }
    if (cur->right) {
      q.push(cur->right);
    }
    cout << q.front()->val << " ";
    q.pop();
  }
}

// 035833100
vector<vector<int>> zigzagLevelOrder(TreeNode *root) {
  if (!root) {
    return {};
  }

  deque<TreeNode *> q;
  q.push_back(root);
  vector<vector<int>> result;

  bool forward = true;
  while (!q.empty()) {
    int size = q.size();
    vector<int> levelVals;

    for (int i = 0; i < size; ++i) {
      TreeNode *cur;
      if (forward) {
        cur = q.front();
        q.pop_front();

        if (cur->left) {
          q.push_back(cur->left);
        }
        if (cur->right) {
          q.push_back(cur->right);
        }

      } else {
        cur = q.back();
        q.pop_back();

        if (cur->right) {
          q.push_front(cur->right);
        }
        if (cur->left) {
          q.push_front(cur->left);
        }
      }

      levelVals.push_back(cur->val);
    }

    result.push_back(levelVals);
    forward = !forward;
  }

  return result;
}

int main() {
  vector<int> input = {3, 9, 20, 1, 2, 15, 7};
  TreeNode *root = buildTree(input, 0);

  vector<vector<int>> peri = zigzagLevelOrder(root);
  for (int i = 0; i < peri.size(); i++) {
    for (int j = 0; j < peri[i].size(); j++) {
      cout << peri[i][j] << " ";
    }
    cout << '\n';
  }

  return 0;
}
