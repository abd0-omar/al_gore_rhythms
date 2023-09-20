#include <algorithm>
#include <iostream>
#include <memory>
#include <unordered_set>
using namespace std;

int unique_anagrams(const string &str) {
  // aabade
  unordered_set<string> set;
  for (int i = 0; i < str.size(); i++) {
    for (int j = i; j < str.size(); j++) {
      string substr = str.substr(i, j - i + 1);
      // set.insert(str.substr(i, j - i + 1));
      sort(substr.begin(), substr.end());
      set.insert(substr);
    }
  }
  return set.size();
}

int main() {
  string x = "abcba";

  int y = unique_anagrams(x);

  cout << y << '\n';
}
