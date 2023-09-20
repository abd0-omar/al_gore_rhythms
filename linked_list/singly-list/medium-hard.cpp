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

  pair<Node *, Node *> move_to_end(Node *curr, Node *prev) {
    if (!prev) { // head
      head = curr->next;
      tail->next = curr;
      tail = curr;
      tail->next = nullptr;
      return {nullptr, head};
    } else if (curr == tail) { // tail
      return {prev, curr};
    } else { // middle
      prev->next = curr->next;
      tail->next = curr;
      tail = curr;
      tail->next = nullptr;
      return {prev, prev->next};
      // return prev->next;
    }
  }

  // void move_to_back(int key) {
  //   if (!head) {
  //     return;
  //   }
  //
  //   Node *curr = head;
  //   Node *prev = nullptr;
  //   int count = 0;
  //
  //   while (curr && count < length) {
  //
  //     if (curr->data == key) {
  //       Node *next = move_to_end(curr, prev);
  //       curr = next;
  //     } else {
  //       prev = curr;
  //       curr = curr->next;
  //     }
  //     count++;
  //   }
  // }

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

  // void arrange_odd_then_even() { // can be solved using stack but will cost
  //                                // O(n/2) -> O(n) space
  //   Node *even_curr = head->next;
  //   Node *even_prev = head;
  //   while (even_curr) {
  //     auto [prev, next] = move_to_end(even_curr, even_prev);
  //     even_curr = next;
  //     even_prev = prev;
  //     // even_curr = next;
  //     // even_prev =
  //     // even_prev = even_curr;
  //     // even_curr = even_curr->next;
  //   }
  //   debug_verify_data_integrity();
  // }

  void arrange_odd_then_even() {
    if (length <= 2)
      return;
    Node *first_even = head->next;
    Node *odd = head;

    while (odd->next && odd->next->next) {
      Node *even = odd->next;
      odd->next = odd->next->next;
      even->next = even->next->next;

      odd = odd->next;

      if (length % 2 == 1) {
        tail = even;
      }
    }

    odd->next = first_even;
    debug_verify_data_integrity();
  }

  void insert_alternate(LinkedList &another) {
    Node *curr = head;
    Node *next = head->next;

    Node *cur = another.head;
    Node *nxt = another.head->next;
    if (!another.length) {
      return;
    }
    if (!length) {
      // copy data
      head = another.head;
      tail = another.tail;
      length = another.length;
      debug_data = another.debug_data;
      return;
    }
    // if len another > len main
    while (curr && cur) {

      Node *temp_next = curr->next;
      Node *temp_nxt = cur->next;

      curr->next = cur;
      cur->next = temp_next;

      curr = temp_next;
      cur = temp_nxt;
      if (curr == tail) {
        tail = another.tail;
        curr->next = cur;
        break;
      }
    }
  }
  // else {
  //     while (curr->next->next && curr && cur) {
  //
  //       Node *temp_next = curr->next;
  //       Node *temp_nxt = cur->next;
  //
  //       curr->next = cur;
  //       cur->next = temp_next;
  //
  //       curr = temp_next;
  //       cur = temp_nxt;
  //     }
  //
  //     if (cur) {
  //       Node *temp_next = curr->next;
  //       Node *temp_nxt = cur->next;
  //       curr->next = cur;
  //       cur->next = temp_next;
  //       temp_next->next = temp_nxt;
  //     }
  //     tail = another.tail;
  // }

  // if (cur) {
  //   curr->next = cur;
  //   tail = another.tail;
  // }
  // }

  void add_num(LinkedList &another) {

    if (!another.length)
      return;

    int carry = 0;
    int myvalue, hisvalue;

    Node *cur = head;
    Node *acur = another.head;

    while (cur || acur) {
      myvalue = 0, hisvalue = 0;
      if (cur)
        myvalue = cur->data;
      if (acur) {
        hisvalue = acur->data;
        acur = acur->next;
      }

      myvalue += hisvalue + carry;
      carry = myvalue / 10;
      myvalue %= 10;

      if (cur) {
        cur->data = myvalue;
        cur = cur->next;
      } else {
        insert_end(myvalue);
      }
    }
    if (carry) {
      insert_end(carry);
    }
  }

  void rmv_duplicates() {
    // frequency array approach can be done here
    Node *cur = head;
    Node *nxt = head->next;
    Node *prv = nullptr;
    int val = -1;

    while (nxt) {
      if (nxt->data == cur->data || cur->data == val) {
        val = cur->data;
        if (cur == head) {
          delete_front();
          cur = head;
        } else {
          prv->next = nxt;
          delete_node(cur);
          cur = nxt;
        }
      } else {
        prv = cur;
        cur = nxt;
      }
      nxt = nxt->next;
    }
    if (val == cur->data) { // last duplicate
      if (cur == head)
        delete_front();
      else {
        prv->next = nullptr;
        delete_node(cur);
        tail = prv;
      }
    }
    if (!head) {
      tail = head;
    }
    debug_verify_data_integrity();
  }

  tuple<Node *, Node *, Node *> reverse_k(Node *c_head, int k) {
    Node *p = c_head;
    Node *c = c_head->next;
    Node *tail_t = p;
    for (int s = 0; s < k - 1 && c; s++) {
      Node *n = c->next;
      c->next = p;
      p = c;
      c = n;
    }
    // tuple<Node *, Node *, Node *> mytuple(tail_t, p, c);
    // return mytuple;
    return make_tuple(tail_t, p, c);
  }

  void reverse_chains(int k) {
    if (length <= 1 || k == 1)
      return;

    Node *c = head;
    Node *last_tail = nullptr;
    head = nullptr;

    while (c) {
      tuple mytuple = reverse_k(c, k);
      Node *tail_chain;
      Node *head_chain;
      Node *next_head_chain;
      tie(tail_chain, head_chain, next_head_chain) = mytuple;
      c = next_head_chain;
      tail = tail_chain;

      if (!head)
        head = head_chain;
      else
        last_tail->next = head_chain;
      last_tail = tail_chain;
    }
    tail->next = nullptr;
  }
};

void test1() {
  cout << "\n\ntest1\n";
  LinkedList list;

  list.insert_end(1);
  list.insert_end(2);
  list.insert_end(3);
  list.insert_end(4);
  list.insert_end(5);
  list.insert_end(6);
  list.reverse_chains(6);
  // some actions
  list.print();

  string expected = "6 5 4 3 2 1";
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
  list.insert_end(5);
  list.insert_end(6);
  list.reverse_chains(3);
  // some actions
  list.print();

  string expected = "3 2 1 6 5 4";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

void test3() {
  cout << "\n\ntest3\n";
  LinkedList list;

  list.insert_end(1);
  list.insert_end(2);
  list.insert_end(3);
  list.insert_end(4);
  list.insert_end(5);
  list.insert_end(6);
  list.insert_end(7);
  list.reverse_chains(2);
  // some actions
  list.print();

  string expected = "2 1 4 3 6 5 7";
  string result = list.debug_to_string();
  if (expected != result) {
    cout << "no match:\nExpected: " << expected << "\nResult  : " << result
         << "\n";
    assert(false);
  }
  list.debug_print_list("********");
}

int main() {
  test1();
  test2();
  test3();

  cout << "\n\nNO RTE\n"; // must see it, otherwise RTE
  return 0;
}
