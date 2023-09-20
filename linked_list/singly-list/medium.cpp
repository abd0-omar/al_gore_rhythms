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
    for (Node *cur = head; cur; cur = cur->next, len++)
      assert(len < 10000); // Consider infinite cycle?
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

  void insert_front(int value) {
    Node *item = new Node(value);
    add_node(item);

    item->next = head;
    head = item;

    if (length == 1)
      tail = head;
  }
  void delete_front() {
    assert(length);
    Node *cur = head->next;
    delete_node(head);
    head = cur;
  }
  ////////////////////////////////////////////////////////////

  void swap_head_tail() {
    if (!head || !head->next)
      return;
    if (!head->next->next) {
      swap(head, tail);
      head->next = tail;
      tail->next = nullptr;
      return;
    }
    Node *curr = head->next;
    Node *prev = head;

    while (curr->next) {
      prev = curr;
      curr = curr->next;
    }
    tail->next = head->next;
    prev->next = head;
    head->next = nullptr;

    swap(head, tail);
    debug_verify_data_integrity();
  }

  void left_rotate(int k) {
    if (!head || !head->next)
      return;
    int small_k = k % length;
    if (small_k == length)
      return;
    while (small_k-- > 0) {
      Node *curr = head->next;
      tail->next = head;
      head->next = nullptr;
      head = curr;
      tail = tail->next;
    }
    // get_nth solution is much better
  }

  void del_duplicates() {
    if (!head)
      return;
    int a[101] = {};
    for (Node *curr = head, *prev = nullptr; curr;) {
      if (a[curr->data] == 1) {
        if (curr == tail) {
          tail = prev;
        }
        prev->next = curr->next;
        delete_node(curr);
        curr = prev->next;
      } else {
        a[curr->data]++;
        prev = curr;
        curr = curr->next;
      }
    }
  }

  void delete_next_node(Node *node) {
    // Delete the next of the current node
    // Handle if next is tail case
    assert(node);

    Node *to_delete = node->next;
    bool is_tail = to_delete == tail;

    // node->next in middle to delete
    node->next = node->next->next;

    delete_node(to_delete);
    if (is_tail)
      tail = node;
  }

  void del_last_occurence(int target) {
    if (!head) {
      return;
    }
    Node *found = nullptr;
    Node *found_prev = nullptr;
    for (Node *curr = head, *prev = nullptr; curr;
         prev = curr, curr = curr->next) {
      if (curr->data == target) {
        found = curr;
        found_prev = prev;
      }
    }
    if (found) {
      if (found == head) {
        delete_front();
      } else {
        delete_next_node(found_prev);
      }
    }
  }

  Node *move_to_end(Node *curr, Node *prev) {
    if (!prev) { // head
      head = curr->next;
      tail->next = curr;
      tail = curr;
      tail->next = nullptr;
      return head;
    } else if (curr == tail) { // tail
      return curr;
    } else { // middle
      prev->next = curr->next;
      tail->next = curr;
      tail = curr;
      tail->next = nullptr;
      return prev->next;
    }
  }

  void move_to_back(int key) {
    if (!head) {
      return;
    }

    Node *curr = head;
    Node *prev = nullptr;
    int count = 0;

    while (curr && count < length) {

      if (curr->data == key) {
        Node *next = move_to_end(curr, prev);
        curr = next;
      } else {
        prev = curr;
        curr = curr->next;
      }
      count++;
    }
  }

  int max(Node *curr = nullptr, bool is_first_call = true) {
    if (is_first_call) {
      curr = head;
    }

    if (!curr)
      return -1;
    int max_num = curr->data;
    max_num = std::max(max_num, max(curr->next, false));

    return max_num;
  }
};

int main() {

  LinkedList list;

  cout << list.max() << "\n"; // -2147483648
  list.insert_end(6);
  list.insert_end(10);
  list.insert_end(8);
  cout << list.max() << "\n"; // 10
  list.insert_end(15);
  cout << list.max() << "\n"; // 15

  return 0;
}
