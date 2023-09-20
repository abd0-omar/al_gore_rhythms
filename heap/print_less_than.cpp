#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

class MinHeap {
private:
  int capacity{1000};

  int *array{};
  int size{};

  int left(int node) {
    int p = 2 * node + 1;
    if (p >= size)
      return -1;
    return p;
  }
  int right(int node) {
    int p = 2 * node + 2;
    return p >= size ? -1 : p;
  }
  int parent(int node) { return node == 0 ? -1 : (node - 1) / 2; }

  void heapify_up(int child_pos) {
    // stop when parent is smaller (or no parent)
    int par_pos = parent(child_pos);
    if (child_pos == 0 || array[par_pos] < array[child_pos])
      return;

    swap(array[child_pos], array[par_pos]);
    heapify_up(par_pos);
  }

  void heapify() { // O(n)
    for (int i = size / 2 - 1; i >= 0; --i)
      heapify_down(i);
  }

  void heapify_down(int parent_pos) { // O(logn)
    int child_pos = left(parent_pos);
    int right_child = right(parent_pos);

    if (child_pos == -1) // no children
      return;
    // is right smaller than left?
    if (right_child != -1 && array[right_child] < array[child_pos])
      child_pos = right_child;

    if (array[parent_pos] > array[child_pos]) {
      swap(array[parent_pos], array[child_pos]);
      heapify_down(child_pos);
    }
  }

public:
  MinHeap() {
    array = new int[capacity]{};
    size = 0;
  }

  MinHeap(const vector<int> &v) {
    assert((int)v.size() <= capacity);
    array = new int[capacity]{};
    size = v.size();

    for (int i = 0; i < (int)v.size(); ++i)
      array[i] = v[i];

    heapify();
  }

  ~MinHeap() {
    delete[] array;
    array = nullptr;
  }

  int top() {
    assert(!isempty());
    return array[0];
  }

  void push(int key) {
    assert(size + 1 <= capacity);
    array[size++] = key;
    heapify_up(size - 1);
  }

  void print_less_than(int val, int par_pos = 0) {
    if (par_pos == -1 || array[par_pos] > val) {
      return;
    }

    cout << array[par_pos] << " ";

    print_less_than(val, left(par_pos));
    print_less_than(val, right(par_pos));
  }

  bool is_heap(int par_pos = 0) {
    if (par_pos == -1) {
      return true;
    }

    if ((left(par_pos) != -1 && array[left(par_pos)] > array[par_pos]) ||
        (right(par_pos) != -1 && array[right(par_pos)] > array[par_pos])) {
      return false;
    }

    return is_heap(left(par_pos)) && is_heap(right(par_pos));
  }

  // void print_less_than(int val, int parent_pos = 0) {
  //   if (parent_pos == -1 || array[parent_pos] >= val)
  //     return;
  //
  //   cout << array[parent_pos] << " ";
  //
  //   print_less_than(val, left(parent_pos));
  //   print_less_than(val, right(parent_pos));
  // }

  void pop() {
    assert(!isempty());
    array[0] = array[--size];
    heapify_down(0);
  }

  bool isempty() { return size == 0; }
};

void creat_heap_nlogn() {
  MinHeap heap;

  vector<int> v{2, 17, 22, 10, 8, 37, 14, 19, 7, 6, 5, 12, 25, 30};

  for (int i = 0; i < (int)v.size(); ++i)
    heap.push(v[i]);

  heap.print_less_than(10); // 2 5 8 6 7
  cout << "\n";

  // h.print();
  while (!heap.isempty()) {
    cout << heap.top() << " ";
    heap.pop();
  }
}

void creat_heap_n() {
  vector<int> v{2, 17, 22, 10, 8, 37, 14, 19, 7, 6, 5, 12, 25, 30};
  MinHeap heap(v);

  // h.print();
  while (!heap.isempty()) {
    cout << heap.top() << " ";
    heap.pop();
  }
}

int main() {
  creat_heap_nlogn();

  return 0;
}
