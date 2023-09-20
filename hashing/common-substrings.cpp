#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

int count_substrings_match(const string &str1, const string &str2) {
  unordered_set<string> s1;

  for (int i = 0; i < (int)str1.size(); ++i) {
    for (int j = i; j < (int)str1.size(); ++j) {
      s1.insert(str1.substr(i, j - i + 1));
    }
  }

  unordered_set<string> s2;
  for (int i = 0; i < (int)str2.size(); ++i) {
    for (int j = i; j < (int)str2.size(); ++j) {
      s2.insert(str2.substr(i, j - i + 1));
    }
  }

  int count = 0;
  for (string str : s1)
    count += s2.count(str);

  return count;
}
int main() {}
