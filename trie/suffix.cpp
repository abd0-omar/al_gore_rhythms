#include <iostream>
using namespace std;

struct Trie {
  Trie *child[26]{};
  bool is_leaf{};

  void insert(string &str) {

    Trie *trie = this;

    for (int i = 0; i < str.length(); i++) {
      trie = this;
      for (int j = i; j < str.length(); j++) {

        cout << str << '\n';
        int ch = str[j] - 'a';
        cout << char(ch + 'a') << " " << i << " j: " << j << '\n';

        if (trie->child[ch] == 0) {
          trie->child[ch] = new Trie();
        }
        trie = trie->child[ch];
      }
      trie->is_leaf = true;
    }
  }
  // we can insert all prefixes
  // think about simple solution then reverse
  // doctor's solution is much better

  bool suffix_exist(string &str) {
    Trie *trie = this;
    for (int i = 0; i < str.length(); i++) {
      int ch = str[i] - 'a';

      if (trie->child[ch] == 0) {
        return false;
      }
      trie = trie->child[ch];
    }
    return trie->is_leaf;
  }
};

int main() {
  Trie *trie = new Trie();
  string s1 = "abdo";
  string s2 = "bd";
  trie->insert(s1);
  bool output = trie->suffix_exist(s2);
  cout << output << "\n";
}
