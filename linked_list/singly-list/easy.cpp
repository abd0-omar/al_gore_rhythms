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
      return;
    }

    assert(head != nullptr);
    assert(tail != nullptr);
    assert(tail->next == nullptr);

    if (length == 1)
      assert(head == tail);
    else {
      assert(head != tail);

      if (length == 2)
        assert(head->next == tail);
      else if (length == 3) {
        assert(head->next);
        assert(head->next->next == tail);
      }
    }
    int len = 0;
    Node *prev = nullptr;
    for (Node *cur = head; cur; prev = cur, cur = cur->next, len++)
      assert(len < 10000); // Consider infinite cycle?

    assert(length == len);
    assert(length == (int)debug_data.size());
    assert(prev == tail); // last node is tail
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

    debug_verify_data_integrity();
  }

  void insert_front(int value) {
    Node *item = new Node(value);
    add_node(item);

    if (!head)
      head = tail = item;
    else {
      item->next = head;
      head = item;
    }

    debug_verify_data_integrity();
  }

  void delete_front() {
    if (!head) {
      return;
    }

    Node *temp = head->next;
    delete_node(head);
    head = temp;
  }

  Node *get_nth(int n) {
    int counter = 1;
    Node *curr = head;
    while (curr) {
      if (counter == n) {
        return curr;
      }
      counter++;
      curr = curr->next;
    }
    return nullptr;
  }

  Node *get_nth_back(int n) {
    int counter = length + 1;
    for (Node *curr = head; curr; curr = curr->next) {
      if (--counter == n) {
        return curr;
      }
    }
    return nullptr;
  }

  bool is_same(const LinkedList &l2) {
    Node *curr1 = head;
    Node *curr2 = l2.head;

    while (curr1 && curr2) {
      if (curr1->data != curr2->data)
        return false;

      curr1 = curr1->next;
      curr2 = curr2->next;
    }

    if (curr1 == nullptr && curr2 == nullptr)
      return true;
    else
      return false;
  }

  void add_element(int value) {
    Node *newNode = new Node(value);
    newNode->next = head;
    head = newNode;
  }

  Node *get_tail() {
    Node *curr = head;
    while (curr && curr->next) {
      curr = curr->next;
    }
    return curr;
  }

  void print2() {
    Node *curr = head;
    while (curr) {
      std::cout << curr->data << " ";
      curr = curr->next;
    }
    std::cout << std::endl;
  }

  ~LinkedList() {
    Node *curr = head;
    while (curr) {
      Node *next = curr->next;
      delete curr;
      curr = next;
    }
  }
  ////////////////////////////////////////////////////////////
};

void test1() {
  cout << "\n\ntest1\n";
  LinkedList list;

  list.insert_end(1);
  list.insert_end(2);
  list.insert_end(3);
  // some actions
  list.print();

  string expected = "1 2 3";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test2() {
  cout << "\n\ntest2\n";
  LinkedList list;

  list.insert_end(1);
  list.insert_end(2);
  list.insert_end(3);
  list.insert_end(4);
  // some actions
  list.print();

  string expected = "1 2 3 4";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test3() {
  cout << "\n\ntest2\n";
  LinkedList list;

  list.insert_end(6);
  list.insert_end(10);
  list.insert_end(8);
  list.insert_end(15);
  list.insert_front(7);
  list.insert_front(5);
  list.insert_front(1);
  // some actions
  list.print();

  string expected = "1 5 7 6 10 8 15";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test4() {
  cout << "\n\ntest2\n";
  LinkedList list;

  list.insert_end(6);
  list.insert_end(10);
  list.insert_end(8);
  list.insert_end(15);
  list.delete_front();
  // some actions
  list.print();

  string expected = "10 8 15";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test5() {
  cout << "\n\ntest2\n";
  LinkedList list;

  list.insert_end(6);
  list.insert_end(10);
  list.insert_end(8);
  list.insert_end(15);
  cout << list.get_nth(2)->data << '\n';
  cout << list.get_nth_back(2)->data << '\n';
  // some actions
  list.print();

  string expected = "6 10 8 15";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test_is_same() {
  cout << "\n\nTesting is_same\n";

  // Test Case 1
  LinkedList list1;
  list1.insert_end(1);
  list1.insert_end(2);
  list1.insert_end(3);

  LinkedList list2;
  list2.insert_end(1);
  list2.insert_end(2);
  list2.insert_end(3);

  cout << "List 1: ";
  list1.print();

  cout << "List 2: ";
  list2.print();

  if (list1.is_same(list2))
    cout << "Test Case 1 Passed: The lists are the same.\n";
  else
    cout << "Test Case 1 Failed: The lists are different.\n";

  // Test Case 2
  LinkedList list3;
  list3.insert_end(1);
  list3.insert_end(2);
  list3.insert_end(3);

  LinkedList list4;
  list4.insert_end(1);
  list4.insert_end(2);
  list4.insert_end(3);
  list4.insert_end(3);

  cout << "List 3: ";
  list3.print();

  cout << "List 4: ";
  list4.print();

  if (!list3.is_same(list4))
    cout << "Test Case 2 Passed: The lists are different.\n";
  else
    cout << "Test Case 2 Failed: The lists are the same.\n";
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();
  test_is_same();

  // must see it, otherwise RTE
  cout << "\n\nNO RTE\n";

  return 0;
}
