#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

class BinaryTree {
private:
  int data{};
  BinaryTree *left{};
  BinaryTree *right{};

public:
  BinaryTree(int data) : data(data) {}

  void print_inorder() {
    if (left)
      left->print_inorder();
    cout << data << " ";
    if (right)
      right->print_inorder();
  }

  int tree_max() {
    int max_val = data;

    if (left && left->tree_max() > max_val)
      max_val = left->tree_max();

    if (right && right->tree_max() > max_val)
      max_val = right->tree_max();

    return max_val;
  }

  int tree_height() {
    int height = 0;

    if (left) {
      height = max(height, 1 + left->tree_height());
    }

    if (right) {
      height = max(height, 1 + right->tree_height());
    }

    return height;
  }

  int tree_count() {
    int count = 1;

    if (left) {
      count += left->tree_count();
    }

    if (right) {
      count += right->tree_count();
    }

    return count;
  }

  int tree_count_leafs() {

    int count = 0;

    if (!left && !right) {
      return 1;
    }

    if (left) {
      count += left->tree_count_leafs();
    }

    if (right) {
      count += right->tree_count_leafs();
    }

    return count;
  }

  bool is_exist(int value) {
    bool flag = false;

    if (data == value) {
      return flag = true;
    }

    if (!flag && left) {
      flag = left->is_exist(value);
    }

    if (!flag && right) {
      flag = right->is_exist(value);
    }

    return flag;
  }

  bool is_perf() {
    bool flag = false;

    if (!left && !right) {
      return flag = true;
    }

    if (left && right) {
      return left->is_perf() && right->is_perf();
    }

    return flag;
  }

  void add(vector<int> values, vector<char> direction) {
    assert(values.size() == direction.size());
    BinaryTree *current = this;
    // iterate on the path, create all necessary nodes
    for (int i = 0; i < (int)values.size(); ++i) {
      if (direction[i] == 'L') {
        if (!current->left)
          current->left = new BinaryTree(values[i]);
        else
          assert(current->left->data == values[i]);
        current = current->left;
      } else {
        if (!current->right)
          current->right = new BinaryTree(values[i]);
        else
          assert(current->right->data == values[i]);
        current = current->right;
      }
    }
  }
};

int main() {
  BinaryTree tree(1);
  tree.add({2, 4, 7}, {'L', 'L', 'L'});
  tree.add({2, 4, 8}, {'L', 'L', 'R'});
  tree.add({2, 5, 9}, {'L', 'R', 'R'});
  tree.add({3, 6, 10}, {'R', 'R', 'L'});

  tree.print_inorder();
  // 7 4 8 2 5 9 1 3 10 6
  cout << "\n";

  int max = tree.tree_max();
  cout << max;
  cout << "\n";

  int height = tree.tree_height();
  cout << height;
  cout << "\n";

  int count = tree.tree_count();
  cout << count;
  cout << "\n";

  int count_leaf = tree.tree_count_leafs();
  cout << count_leaf;
  cout << "\n";

  bool flag = tree.is_exist(12);
  cout << flag;
  cout << "\n";
  cout << "\n";

  BinaryTree tree2(2);
  tree2.add({4}, {'L'});
  tree2.add({6}, {'R'});

  bool flagz = tree2.is_perf();
  cout << flagz;
  cout << "\n";

  return 0;
}
