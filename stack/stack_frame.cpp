#include <cassert>
#include <iostream>
using namespace std;

struct StackElement {
  int n;
  int result = -1;

  StackElement(int n = 1, int result = -1) : n(n), result(result) {}
  bool is_computed() { return result >= 0; }
};

class Stack {
private:
  int size{};
  int top{-1};
  StackElement *array{};

public:
  Stack(int size) : size(size), top(-1) { array = new StackElement[size]{}; }

  ~Stack() { delete[] array; }

  void push(StackElement x) {
    assert(!isFull());
    array[++top] = x;
  }

  StackElement pop() {
    assert(!isEmpty());
    return array[top--];
  }

  StackElement peek() {
    assert(!isEmpty());
    return array[top];
  }

  bool isFull() { return top == size - 1; }

  bool isEmpty() { return top == -1; }
};

int factorial_stack(int n) {
  if (n <= 1)
    return 1;

  Stack st(n);
  st.push(StackElement(n));
  StackElement cur(1);

  while (!st.isEmpty()) {
    cur = st.peek();

    if (!cur.is_computed()) {
      if (cur.n <= 1) {
        cur.result = 1;
        st.pop();
        st.push(cur);
      } else
        st.push(StackElement(cur.n - 1));
    } else {
      cur = st.pop();
      if (!st.isEmpty()) {
        StackElement parent = st.pop();
        parent.result = parent.n * cur.result;
        st.push(parent);
      }
    }
  }
  return cur.result;
}

int main() {
  for (int i = 0; i < 10; ++i) {
    cout << i << " " << factorial_stack(i) << "\n";
  }

  return 0;
}
