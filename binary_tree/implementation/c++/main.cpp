#include <iostream>
using namespace std;

struct Node {
  int data{};
  Node *left{};
  Node *right{};
  Node(int data) : data(data) {}
};

int main() {
  Node *node0 = new Node(1);
  Node *node1 = new Node(2);
  Node *node2 = new Node(3);
  Node *node3 = new Node(4);
  Node *node4 = new Node(5);
  Node *node5 = new Node(6);
  Node *node6 = new Node(7);

  node0->left = node1;
  node0->right = node2;

  node1->left = node3;
  node1->right = node4;

  node4->right = node5;

  node2->right = node6;

  node5->left = node6;

  cout << node0->left->right->right->data << "\n"; // 7
  cout << node1->right->right->data << "\n";       // 7
  cout << node4->right->data << "\n";              // 7
  cout << node6->data << "\n";                     // 7

  cout << node0->right->right->data << "\n";       // 6
  cout << node0->right->right->left->data << "\n"; // 8
  cout << node0->right->right->right << "\n";      // 0x00

  return 0;
}
