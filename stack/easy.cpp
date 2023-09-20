#include <cassert>
#include <cmath>
#include <iostream>
#include <stack>
#include <string>
#include <unordered_map>
#include <valarray>
#include <vector>
using namespace std;

// simple trick to avoid re-changing the class
// nice
typedef char type;

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

  void display() {
    for (int i = top; i >= 0; i--)
      cout << array[i] << " ";
    cout << "\n";
  }
};

string reverse_subwords(string line) {
  Stack stack(line.length());
  line += ' ';
  string res = "";
  for (int i = 0; i < line.length(); i++) {
    if (line[i] == ' ') {
      while (!stack.isEmpty()) {
        res += stack.pop();
      }
      res += ' ';
      continue;
    }
    stack.push(line[i]);
  }
  res.pop_back();
  line.pop_back();
  return res;
}

int reverse_num(int num) {
  Stack s(100);
  while (num != 0) {
    int mod = num % 10;
    s.push(mod);
    num /= 10;
  }
  int malt = 1;
  int res = 0;

  while (!s.isEmpty()) {
    res = s.pop() * malt + res, malt *= 10;
  }
  return res;
}

bool isValid(string str) {
  Stack s(str.length());

  unordered_map<char, char> m;
  m['('] = ')';
  m['['] = ']';
  m['{'] = '}';

  for (int i = 0; i < str.length(); i++) {

    if (!s.isEmpty() && m[s.peek()] == str[i]) {
      int _popped = s.pop();
      continue;
    }

    s.push(str[i]);
  }

  return s.isEmpty();
}

string removeDuplicates(string s) {
  Stack st(s.length());

  for (int i = 0; i < s.length(); i++) {
    if (!st.isEmpty() && st.peek() == s[i]) {
      st.pop();
    }

    st.push(s[i]);
  }

  string res = "";
  while (!st.isEmpty()) {
    res += st.pop();
  }
  return res;
}
