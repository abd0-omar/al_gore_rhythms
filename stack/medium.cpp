#include <cassert>
#include <iostream>
#include <stack>
#include <vector>
using namespace std;

// simple trick to avoid re-changing the class
typedef int type;

class Stack {
private:
  int size{};
  int top{};
  type *array{};

public:
  Stack(int size) : size(size), top(-1) { array = new type[size]; }

  ~Stack() { delete[] array; }

  void push(type x) {
    assert(!isFull());
    array[++top] = x;
  }

  type pop() {
    assert(!isEmpty());
    return array[top--];
  }

  type peek() {
    assert(!isEmpty());
    return array[top];
  }

  int isFull() { return top == size - 1; }

  int isEmpty() { return top == -1; }

  void display_reversed() {
    for (int i = 0; i <= top; i++)
      cout << array[i] << " ";
    cout << "\n";
  }

  void display() {
    for (int i = top; i >= 0; i--)
      cout << array[i] << " ";
    cout << "\n";
  }

  void ins_at_bot(int x) {
    if (isEmpty()) {
      push(x);
    } else {
      int y = pop();

      ins_at_bot(x);
      push(y);
    }
  }

  void reverse() {
    if (isEmpty()) {
      return;
    }

    int y = pop();

    reverse();

    ins_at_bot(y);
  }
};

vector<int> asteroidCollision(vector<int> &asteroids) {
  vector<int> st;

  for (int i = 0; i < asteroids.size(); i++) {
    bool is_destroyed = false;
    while (!st.empty() && st.back() > 0 && asteroids[i] < 0) {
      if (asteroids[i] * -1 > st.back()) {
        st.pop_back();
        continue;
      } else if (st.back() == -1 * asteroids[i]) {
        st.pop_back();
        is_destroyed = true;
        break;
      }
    }
    if (!is_destroyed) {
      st.push_back(asteroids[i]);
    }
  }
  return st;
}

int score(string s) {
  stack<int> st;
  for (int c : s) {
    if (c == '(') {
      st.push(0);
    } else {
      if (st.top() == 0) {
        st.pop();
        st.push(1);
      } else {
        int count = 0;
        while (st.top() != 0) {
          count += st.top();
          st.pop();
        }
        st.pop();
        count *= 2;
        st.push(count);
      }
    }
  }
  long long ans = 0;
  while (!st.empty()) {
    ans += st.top();
    st.pop();
  }
  return ans;
}

int score_drs_better_version(string s) {
  stack<int> st;
  st.push(0); // whole parent aka parent of all parents

  for (int c : s) {
    if (c == '(') {
      st.push(0);
    } else {
      int last = st.top();
      st.pop();
      if (last == 0) {
        last = 1;
      } else {
        last *= 2;
      }
      int parent = last + st.top();
      st.pop();
      st.push(parent);
    }
  }
  return st.top();
}

// two techniques: simpling technique and reversing technique

// my solution to daily_temperatures (next larger elem with a twist) using
// rust lang(for fun) in ./daily-temp/src/main.rs

int main() {

  Stack stk(10);
  stk.push(1);
  stk.push(2);
  stk.push(3);
  stk.display(); // 3 2 1
  stk.reverse();
  stk.display(); // 1 2 3

  return 0;
}
