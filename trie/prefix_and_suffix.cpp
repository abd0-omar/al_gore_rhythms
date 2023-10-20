#include <cstring>
#include <iostream>
#include <vector>
using namespace std;

struct Trie {
  Trie *child[26]{};
  bool is_leaf{};
  vector<int> indices;

  void insert(string const &str) {
    Trie *trie = this;
    for (int i = 0; i < str.size(); i++) {
      int cur = str[i] - 'a';
      if (trie->child[cur] == 0) {
        trie->child[cur] = new Trie();
      }
      trie = trie->child[cur];
    }
    trie->is_leaf = true;
  }

  void insert_vec(vector<string> &vec) {
    for (int i = vec.size() - 1; i >= 0; i--) {
      Trie *trie = this;

      for (int j = 0; j < vec[i].size(); j++) {
        int cur = vec[i][j] - 'a';

        if (trie->child[cur] == 0) {
          trie->child[cur] = new Trie();
        }

        trie->child[cur]->indices.push_back(i);
        trie = trie->child[cur];
      }

      trie->is_leaf = true;
    }
    this->insert_vec_reversed(vec);
  }

  void insert_vec_reversed(vector<string> &vec) {
    for (int i = vec.size() - 1; i >= 0; i--) {
      Trie *trie = this;

      for (int j = vec[i].size() - 1; j >= 0; j--) {
        int cur = vec[i][j] - 'a';

        if (trie->child[cur] == 0) {
          trie->child[cur] = new Trie();
        }

        trie->child[cur]->indices.push_back(i);
        trie = trie->child[cur];
      }

      trie->is_leaf = true;
    }
  }

  vector<int> get_positions(const string &str) {
    Trie *trie = this;

    for (int i = 0; i < str.size(); i++) {
      int cur = str[i] - 'a';

      if (trie->child[cur] == 0) {
        return vector<int>();
      }

      trie = trie->child[cur];
    }

    return trie->indices;
  }

  void insert_prefixes(string const &str) {
    Trie *trie = this;
    for (int i = 0; i < str.size(); i++) {
      string word = str.substr(i, str.size());
      for (int j = 0; j < word.size(); j++) {
        int cur = word[j] - 'a';
        if (trie->child[cur] == 0) {
          trie->child[cur] = new Trie;
        }
        trie = trie->child[cur];
      }
      trie->is_leaf = true;
    }
  }

  bool is_prefix(string const &str) {
    Trie *trie = this;
    for (int i = 0; i < str.size(); i++) {
      int cur = str[i] - 'a';
      if (trie->child[cur] == 0) {
        return false;
      }
    }
    return true;
  }

  int f(string prefix, string suffix) {
    Trie *trie = this;
    vector<int> positions = trie->get_positions(prefix);

    for (int i = 0; i < positions.size(); i++) {
      cout << positions[i] << ", ";
    }
    cout << '\n';

    for (int i = 0; i < positions.size(); i++) {
      // if ()
    }

    return -1;
  }
};

int main() {
  Trie *trie = new Trie();
  vector<string> vec{"apple", "aae", "apple", "banana"};
  trie->insert_vec(vec);
  trie->f("ap", "e");
}
