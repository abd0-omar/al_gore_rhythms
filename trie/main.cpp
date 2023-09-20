#include <cstring> // memset
#include <iostream>
using namespace std;

class trie {
private:
  static const int MAX_CHAR = 26;
  trie *child[MAX_CHAR];
  bool isLeaf{};

public:
  trie() {
    // set an array to 0s. Here pointers to null
    memset(child, 0, sizeof(child));
  }

  void insert_iterative(string str) {
    trie *curr_trie = this;
    for (int i = 0; i < str.size(); i++) {
      int curr_char = str[i] - 'a';
      if (child[curr_char] == 0) {
        curr_trie->child[curr_char] = new trie();
      }
      curr_trie = curr_trie->child[curr_char];
    }
    curr_trie->isLeaf = true;
  }

  bool word_exist_iterative(string str) {
    trie *curr_trie = this;

    for (int i = 0; i < str.size(); i++) {
      int curr_char = str[i] - 'a';
      if (curr_trie->child[curr_char] == 0) {
        return false;
      }
      curr_trie = curr_trie->child[curr_char];
    }
    return curr_trie->isLeaf;
  }

  void insert(string str, int idx = 0) {
    if (idx == (int)str.size())
      isLeaf = 1;
    else {
      int cur = str[idx] - 'a';
      if (child[cur] == 0)
        child[cur] = new trie();
      child[cur]->insert(str, idx + 1);
    }
  }

  bool word_exist(string str, int idx = 0) {
    if (idx == (int)str.size())
      return isLeaf; // there is a string marked here

    int cur = str[idx] - 'a';
    if (!child[cur])
      return false; // such path don't exist

    return child[cur]->word_exist(str, idx + 1);
  }

  bool prefix_exist(string str, int idx = 0) {
    if (idx == (int)str.size())
      return true; // all subword covered

    int cur = str[idx] - 'a';
    if (!child[cur])
      return false; // such path don't exist

    return child[cur]->prefix_exist(str, idx + 1);
  }
  string first_word_prefix(const string &str) {
    //     ● E.g. Assume trie has words {xyz, xyzeA, a, bc}
    // ● Input ⇒ output
    // ○ x ⇒ x [no trie word is prefix for x
    // ○ xyzabc ⇒ xyz is the smallest prefix to xyzabc
    trie *curr_trie = this;

    string ans = "";

    for (int i = 0; i < str.size(); i++) {
      int curr_char = str[i] - 'a';
      if (curr_trie->isLeaf)
        break;
      if (curr_trie->child[curr_char] == 0) {
        return str;
      } else {
        ans += str[i];
      }
      curr_trie = curr_trie->child[curr_char];
    }
    return ans;
  }
};

int main() {
  trie root;

  root.insert("abcd");
  root.insert("xyz");
  root.insert("xyzea");
  root.insert("abf");
  root.insert("xn");
  root.insert("ab");
  root.insert("bcd");

  cout << root.word_exist("xyz") << "\n";
  cout << root.word_exist("xy") << "\n";
  cout << root.prefix_exist("xy") << "\n";
  cout << root.prefix_exist("cd") << "\n";
  cout << root.first_word_prefix("xyzabc") << "\n";

  return 0;
}
