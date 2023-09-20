#include <cassert>
#include <climits>
#include <iostream>

#include <algorithm>
#include <sstream>
#include <vector> // for debug
using namespace std;

struct Node {
  int data{};
  Node *next{};
  Node(int data) : data(data) {}
  ~Node() {
    cout << "Destroy value: " << data << " at address " << this << "\n";
  }
};

class LinkedList {
private:
  Node *head{};
  Node *tail{};
  int length = 0; // let's maintain how many nodes

  vector<Node *> debug_data; // add/remove nodes you use

  void debug_add_node(Node *node) { debug_data.push_back(node); }
  void debug_remove_node(Node *node) {
    auto it = std::find(debug_data.begin(), debug_data.end(), node);
    if (it == debug_data.end())
      cout << "Node does not exist\n";
    else
      debug_data.erase(it);
  }

public:
  // Below 2 deletes prevent copy and assign to avoid this mistake
  LinkedList() {}
  LinkedList(const LinkedList &) = delete;
  LinkedList &operator=(const LinkedList &another) = delete;

  void debug_print_address() {
    for (Node *cur = head; cur; cur = cur->next)
      cout << cur << "," << cur->data << "\t";
    cout << "\n";
  }

  void debug_print_node(Node *node, bool is_seperate = false) {
    if (is_seperate)
      cout << "Sep: ";
    if (node == nullptr) {
      cout << "nullptr\n";
      return;
    }
    cout << node->data << " ";
    if (node->next == nullptr)
      cout << "X ";
    else
      cout << node->next->data << " ";

    if (node == head)
      cout << "head\n";
    else if (node == tail)
      cout << "tail\n";
    else
      cout << "\n";
  }
  void debug_print_list(string msg = "") {
    if (msg != "")
      cout << msg << "\n";
    for (int i = 0; i < (int)debug_data.size(); ++i)
      debug_print_node(debug_data[i]);
    cout << "************\n" << flush;
  }

  string debug_to_string() {
    if (length == 0)
      return "";
    ostringstream oss;
    for (Node *cur = head; cur; cur = cur->next) {
      oss << cur->data;
      if (cur->next)
        oss << " ";
    }
    return oss.str();
  }

  void debug_verify_data_integrity() {
    if (length == 0) {
      assert(head == nullptr);
      assert(tail == nullptr);
    } else {
      assert(head != nullptr);
      assert(tail != nullptr);
      if (length == 1)
        assert(head == tail);
      else
        assert(head != tail);
      assert(!tail->next);
    }
    int len = 0;
    Node *prev = nullptr;
    for (Node *cur = head; cur; prev = cur, cur = cur->next, len++)
      assert(len < 10000); // Consider infinite cycle?
    assert(tail == prev);
    assert(length == len);
    assert(length == (int)debug_data.size());
  }

  ////////////////////////////////////////////////////////////

  void print() {
    for (Node *cur = head; cur; cur = cur->next)
      cout << cur->data << " ";
    cout << "\n";
  }

  // These 2 simple functions just to not forget changing the vector and length
  void delete_node(Node *node) {
    debug_remove_node(node);
    --length;
    delete node;
  }

  void add_node(Node *node) {
    debug_add_node(node);
    ++length;
  }

  void insert_end(int value) {
    Node *item = new Node(value);
    add_node(item);

    if (!head)
      head = tail = item;
    else
      tail->next = item, tail = item;
  }

  ////////////////////////////////////////////////////////////

  void del(int value) {

    Node *first = head;
    if (first && first->data == value) { // head
      head = first->next;
      delete_node(first);
      return;
    }

    Node *curr = head;
    Node *prev = nullptr;
    while (curr) {
      if (curr->data == value) {
        if (curr->next) { // in the middle
          prev = curr->next;
          delete_node(curr);
          return;
        } else { // tail
          delete_node(tail);
          tail = prev;
          return;
        }
      }
      prev = curr;
      curr = curr->next;
    }
  }

  void swap_pairs() {
    Node *curr = head->next;
    Node *prev = head;

    while (curr) {

      swap(curr->data, prev->data);

      prev = curr->next;
      curr = curr->next->next;
    }
  }

  void reverse() {
    Node *curr = head;
    Node *next = nullptr;
    Node *prev = nullptr;
    tail = curr;
    tail->next = nullptr;

    while (curr) {
      next = curr->next;
      curr->next = prev;
      prev = curr;
      curr = next;
    }

    head = prev;
  }

  void delete_even_positions() {
    Node *curr = head->next;
    Node *prev = head;
    while (curr) {
      Node *next_iter = curr->next->next;
      prev->next = curr->next;
      delete_node(curr);
      curr = next_iter;
    }
    if (!curr) {
      tail = prev;
    }
  }

  void inser_sorted(int value) {
    Node *new_node = new Node(value);
    if (!head) { // no head
      head = tail = new_node;
      return;
    }

    Node *curr = head;
    Node *prev = nullptr;

    while (new_node->data > curr->data) {
      prev = curr;
      curr = curr->next;
    }

    if (prev == nullptr) { // smaller than head
      new_node->next = head;
      head = new_node;
      return;
    }

    if (new_node->data > tail->data) { // bigger than tail
      tail->next = new_node;
      tail = new_node;
      return;
    }

    prev->next = new_node; // in the middle
    new_node->next = curr;
  }
};

int main() {
  // test3();
  LinkedList list;

  // Create a linked list: 1 -> 2 -> 3 -> 4 -> 5
  list.insert_end(1);
  list.insert_end(2);
  list.insert_end(3);
  list.insert_end(4);
  list.insert_end(5);

  std::cout << "Original Linked List: ";
  list.print();

  // Swap adjacent pairs of nodes
  list.swap_pairs();

  std::cout << "After Swapping: ";
  list.print();
  // must see it, otherwise RTE
  cout << "\n\nNO RTE\n";

  return 0;
}
