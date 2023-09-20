#include <iostream>
#include <vector>
using namespace std;

void print(vector<int> v) {
  cout << "v: ";
  for (int i = 0; i < v.size(); i++) {
    cout << v[i] << ", ";
  }
  cout << '\n';
}

// 5 lines, sadly
void insertion_descending_with_swap(vector<int> v) {
  for (int i = 1; i < v.size(); i++) {
    int x = i;
    cout << "in while\n";
    while (x > 0 && v[x] > v[x - 1]) {
      swap(v[x], v[x - 1]);
      x--;
      print(v);
    }
    cout << "end of while\n";
    print(v);
  }
}

void insertion(vector<int> v) {
  for (int i = 1; i < v.size(); i++) {
    int cpy = v[i];
    int j = i - 1;
    cout << "in while\n";
    while (j >= 0 && v[j] > cpy) {
      v[j + 1] = v[j];
      j--;
      print(v);
    }
    cout << "end of while\n";
    v[j + 1] = cpy;
    print(v);
  }
}

int main() {
  vector<int> v2{2, 5};
  vector<int> v3{8, 6, 4, 2, 5};
  vector<int> v{5, 2, 4, 6, 1, 3};
  // insertion(v);
  insertion_descending_with_swap(v);
}
