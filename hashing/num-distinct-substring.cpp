#include <iostream>
#include <string>
using namespace std;

struct Trie {
  Trie *child[26]{0};
  bool is_leaf{};
};

void insert(Trie *trie, string &str, int &count, int idx = 0) {
  if (idx == str.length()) {
    if (!trie->is_leaf) {
      trie->is_leaf = true;
      count++;
      cout << "str: " << str << '\n';
    }
    return;
  }

  int cur = str[idx] - 'a';
  if (!trie->child[cur]) {
    trie->child[cur] = new Trie();
  }

  insert(trie->child[cur], str, count, idx + 1);
}

int main() {
  string s = "aaab";

  int count = 0;

  Trie *trie = new Trie();
  for (int i = 0; i < s.length(); i++) {
    string str = "";
    for (int j = i; j < s.length(); j++) {
      str += s[j];
      insert(trie, str, count);
    }
  }

  cout << "count is: " << count << '\n';

  return 0;
}
