#include <iostream>
#include <vector>
using namespace std;

int main() {

  vector<int> vec{5, 2};
  int sz{(int)vec.size()};

  for (int i = 1; i < sz; ++i) {
    int cur{vec[i]};
    int j = i - 1;
    for (; j >= 0; --j) {
      if (cur < vec[j]) {
        vec[j + 1] = vec[j];
      } else {
        cout << "ok";
        vec[j + 1] = cur;
        break;
      }
    }
    // vec[j + 1] = cur;
  }

  for (int i = 0; i < sz; i++) {
    cout << vec[i] << " ";
  }
}
