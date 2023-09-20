#include <iostream>
#include <vector>
using namespace std;
class Solution {
  struct Trie {
    Trie *child[26]{};
    bool is_leaf{};

    void insert(string &str) {
      Trie *curr_trie = this;
      for (int i = 0; i < str.size(); i++) {
        int curr_char = str[i] - 'a';
        if (curr_trie->child[curr_char] == 0) {
          curr_trie->child[curr_char] = new Trie();
        }
        curr_trie = curr_trie->child[curr_char];
      }
      curr_trie->is_leaf = true;
    }

    string smallest_prefix(string &str) {
      Trie *trie = this;

      string ans = "";

      for (int i = 0; i < str.size(); i++) {
        int ch = str[i] - 'a';
        if (trie->is_leaf) {
          return ans;
        }
        if (trie->child[ch] == 0) {
          return str;
        } else {
          ans += str[i];
        }
        trie = trie->child[ch];
      }
      return ans;
    }
  };

public:
  string replaceWords(vector<string> &dictionary, string sentence) {
    // Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was
    // rattled by the battery" Output: "the cat was rat by the bat"
    Trie *trie = new Trie();

    for (int i = 0; i < dictionary.size(); i++) {
      trie->insert(dictionary[i]);
    }

    string rez = "";

    for (int i = 0; i < sentence.size(); i++) {
      string word = "";
      for (int j = i; j <= sentence.size(); j++) {
        if (sentence[j] == ' ' || j == sentence.size()) {
          word += sentence.substr(i, j - i);
          i = j;
          break;
        }
      }
      // rez = i != sentence.size() - 1 ? rez + trie->smallest_prefix(word) + "
      // "
      //                                : rez + trie->smallest_prefix(word);
      rez += trie->smallest_prefix(word) + " ";
    }

    if (rez[rez.size() - 1] == ' ') {
      rez.pop_back();
    }

    return rez;
  }
};
int main() {

  Solution s;
  vector<string> dictionary = {"cat", "bat", "rat"};
  string sentence = "the cattle was rattled by the battery";
  // Output: "the cat was rat by the bat"
  string output = s.replaceWords(dictionary, sentence);
  cout << output;
}
