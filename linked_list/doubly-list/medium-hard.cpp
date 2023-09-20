#include <cassert>
#include <climits>
#include <iostream>

#include <algorithm>
#include <sstream>
#include <tuple>
#include <vector> // for debug
using namespace std;

struct Node {
  int data{};
  Node *next{};
  Node *prev{}; // Previous node!

  Node(int data) : data(data) {}

  void set(Node *next, Node *prev) {
    this->next = next;
    this->prev = prev;
  }

  ~Node() {
    cout << "Destroy value: " << data << " at address " << this << "\n";
  }
};

class LinkedList {
private:
  Node *head{};
  Node *tail{};
  int length = 0;

  // let's maintain how many nodes

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

    if (node->prev == nullptr)
      cout << "X\t";
    else
      cout << node->prev->data << "\t";

    cout << " <= [" << node->data << "]\t => \t";

    if (node->next == nullptr)
      cout << "X\t";
    else
      cout << node->next->data << "\t";

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
      assert(!head->prev);
      assert(!tail->next);
    }
    int len = 0;
    for (Node *cur = head; cur; cur = cur->next, len++) {
      if (len == length - 1) // make sure we end at tail
        assert(cur == tail);
    }

    assert(length == len);
    assert(length == (int)debug_data.size());

    len = 0;
    for (Node *cur = tail; cur; cur = cur->prev, len++) {
      if (len == length - 1) // make sure we end at head
        assert(cur == head);
    }
    cout << "\n";
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

  void link(Node *first, Node *second) {
    if (first)
      first->next = second;
    if (second)
      second->prev = first;
  }

  void insert_end(int value) {
    Node *item = new Node(value);
    add_node(item);

    if (!head)
      head = tail = item;
    else {
      link(tail, item);
      tail = item;
    }
    debug_verify_data_integrity();
  }

  void insert_front(int value) {
    Node *item = new Node(value);
    add_node(item);

    if (!head)
      head = tail = item;
    else {
      link(item, head);
      head = item;
    }
    debug_verify_data_integrity();
  }

  void print_reversed() {
    for (Node *cur = tail; cur; cur = cur->prev)
      cout << cur->data << " ";
    cout << "\n";
  }

  void delete_front() {
    if (!head)
      return;
    Node *cur = head->next;
    delete_node(head);
    head = cur;

    // Integrity change
    if (head)
      head->prev = nullptr;
    else if (!length)
      tail = nullptr;

    debug_verify_data_integrity();
  }

  void delete_end() {
    if (!head)
      return;
    Node *cur = tail->prev;
    delete_node(tail);
    tail = cur;

    // Integrity change
    if (tail)
      tail->next = nullptr;
    else if (!length)
      head = nullptr;

    debug_verify_data_integrity();
  }

  Node *delete_and_link(Node *cur) {
    // remove this node, but connect its neighbors
    Node *ret = cur->prev;
    link(cur->prev, cur->next);
    delete_node(cur);

    return ret;
  }

  void delete_node_with_key(int value) { // O(n) time - O(1) memory
    if (!length)
      cout << "Empty list!\n";
    else if (head->data == value)
      delete_front();
    else {
      for (Node *cur = head; cur; cur = cur->next) {
        if (cur->data == value) {
          cur = delete_and_link(cur);
          if (!cur->next) // we removed last node!
            tail = cur;
          break;
        }
      }
      cout << "Value not found!\n";
    }
    debug_verify_data_integrity();
  }

  void embed_after(Node *node_before, int value) {
    // Add a node with value between node and its next
    Node *middle = new Node(value);
    ++length;
    debug_add_node(middle);

    Node *node_after = node_before->next;
    link(node_before, middle);
    link(middle, node_after);
  }

  void insert_sorted(int value) {
    if (!length || value <= head->data)
      insert_front(value);
    else if (tail->data <= value)
      insert_end(value);
    else {
      // Find the node I am less than. Then I am before it
      for (Node *cur = head; cur; cur = cur->next) {
        if (value <= cur->data) {
          embed_after(cur->prev, value);
          break;
        }
      }
    }
    debug_verify_data_integrity();
  }

  tuple<int, int>
  find_middle() { // we can make another function so that it encapsulate whether
                  // to return one value or two values
                  // nvm bad idea also I use tuple rather than pair because it's
                  // in other langs so easier for others to view it but it
                  // doesn't matter anyway
    if (!head)
      return make_tuple(-1, -1);

    if (!head->next)
      return make_tuple(-1, head->data);

    Node *s = head;
    Node *f = head;
    Node *p = nullptr;

    while (f && f->next) {
      p = s;
      s = s->next;
      f = f->next->next;
    }

    if (f) {
      return make_tuple(p->data, s->data);
    } else {
      return make_tuple(-1, s->data);
    }
  }
  Node *get_nth(int n) {
    int cnt = 0;
    for (Node *cur = head; cur; cur = cur->next)
      if (++cnt == n)
        return cur;

    return nullptr;
  }

  Node *get_nth_back(int n) {
    int cnt = 0;
    for (Node *cur = tail; cur; cur = cur->prev)
      if (++cnt == n)
        return cur;

    return nullptr;
  }
  void swap_kth(int k) {
    if (k <= 0)
      return;

    Node *first = get_nth(k);
    Node *last = get_nth_back(k);

    if (!first || !last || first == last)
      return; // One or both nodes not found or same node

    if (first->prev == last)
      swap(first, last);

    Node *first_prev = first->prev;
    Node *first_next = first->next;

    Node *last_prev = last->prev;
    Node *last_next = last->next;

    if (first->next == last) {
      link(first_prev, last);
      link(last, first);
      link(first, last_next);
    } else {
      link(first_prev, last);
      link(last, first_next);

      link(last_prev, first);
      link(first, last_next);
    }

    if (k == 1)
      swap(head, tail);

    debug_verify_data_integrity();
  }

  void reverse() {
    if (!head || !head->next)
      return;

    Node *p = head;
    Node *c = head->next;

    while (c) {
      Node *n = c->next;

      link(p, c);

      p = c;
      c = n;
    }

    swap(head, tail);

    head->prev = nullptr;
    tail->prev = nullptr;
  }

  void merge_two_sorted_lists(LinkedList &another) {
    if (!head && !another.head)
      return;
    if (!another.head)
      return;
    if (!head) {
      head = another.head;
      tail = another.tail;
      return;
    }
    Node *c = head;
    Node *a = another.head;
    // Node *last;
    bool up; // up == true means the last is at the another ll
    Node *the_head;

    if (head && another.head) {
      if (head->data > another.head->data)
        up = true, the_head = another.head;
      // last = another.head;
      else
        up = false, the_head = head;
      // last = head;
    }

    head = the_head;

    while (c && a) {
      Node *cn = c->next;
      Node *an = a->next;

      if (up) {
        // if (an >= c) {
        //   a->next = an; //which is normal
        // }
        if (!an || c->data < an->data) {
          a->next = c;
          up = false;
        }
        a = an;
      } else {
        if (!cn || a->data < cn->data) {
          c->next = a;
          up = true;
        }
        c = cn;
      }
    }
    if (up) // after iterating for finding the tail
      tail = c;
    else
      tail = a;
  } // my code fails at test 4 and I have to refactor it to work, but I
    // understood the professor's code and I also didn't manage prev as just I
    // was testing
};

void test1() {
  cout << "\n\ntest1\n";
  LinkedList list1;
  list1.insert_end(10);
  list1.insert_end(20);
  list1.insert_end(30);
  list1.insert_end(40);
  list1.insert_end(50);

  LinkedList list2;
  list2.insert_end(15);
  list2.insert_end(17);
  list2.insert_end(22);
  list2.insert_end(24);
  list2.insert_end(35);

  list1.merge_two_sorted_lists(list2);
  list1.print();

  string expected = "10 15 17 20 22 24 30 35 40 50";
  string result = list1.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list1.debug_print_list("********");
}

void test2() {
  cout << "\n\ntest2\n";
  LinkedList list1;
  list1.insert_end(10);
  list1.insert_end(20);
  list1.insert_end(30);
  list1.insert_end(40);
  list1.insert_end(50);

  LinkedList list2;
  list2.insert_end(5);
  list2.insert_end(17);
  list2.insert_end(22);
  list2.insert_end(24);
  list2.insert_end(60);

  list1.merge_two_sorted_lists(list2);
  list1.print();

  string expected = "5 10 17 20 22 24 30 40 50 60";
  string result = list1.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list1.debug_print_list("********");
}

void test3() {
  cout << "\n\ntest3\n";
  LinkedList list1;
  list1.insert_end(10);
  list1.insert_end(20);
  list1.insert_end(30);
  list1.insert_end(40);
  list1.insert_end(50);

  LinkedList list2;
  list2.insert_end(60);

  list1.merge_two_sorted_lists(list2);
  list1.print();

  string expected = "10 20 30 40 50 60";
  string result = list1.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list1.debug_print_list("********");
}

void test4() {
  cout << "\n\ntest4\n";
  LinkedList list1;
  list1.insert_end(60);

  LinkedList list2;
  list2.insert_end(10);
  list2.insert_end(20);
  list2.insert_end(30);
  list2.insert_end(40);
  list2.insert_end(50);

  list1.merge_two_sorted_lists(list2);
  list1.print();

  string expected = "10 20 30 40 50 60";
  string result = list1.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list1.debug_print_list("********");
}

void test5() {
  cout << "\n\ntest4\n";
  LinkedList list1;

  LinkedList list2;
  list2.insert_end(10);
  list2.insert_end(20);
  list2.insert_end(30);
  list2.insert_end(40);
  list2.insert_end(50);

  list1.merge_two_sorted_lists(list2);
  list1.print();

  string expected = "10 20 30 40 50";
  string result = list1.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list1.debug_print_list("********");
}

int main() {
  test1();
  test2();
  test3();
  test4();
  test5();

  // must see it, otherwise RTE
  cout << "\n\nNO RTE\n";

  return 0;
}
