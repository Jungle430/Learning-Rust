// go:build ignore
#include <algorithm>
#include <array>
#include <cctype>
#include <charconv>
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <functional>
#include <iostream>
#include <list>
#include <map>
#include <memory>
#include <numeric>
#include <queue>
#include <ranges>
#include <set>
#include <stack>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

struct ListNode {
  int val;
  ListNode *next;
  ListNode() : val(0), next(nullptr) {}
  ListNode(int x) : val(x), next(nullptr) {}
  ListNode(int x, ListNode *next) : val(x), next(next) {}
};

struct TreeNode {
  int val;
  TreeNode *left;
  TreeNode *right;
  TreeNode() : val(0), left(nullptr), right(nullptr) {}
  TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
  TreeNode(int x, TreeNode *left, TreeNode *right)
      : val(x), left(left), right(right) {}
};

struct TrieNode {
  bool isEnd;
  array<TrieNode *, 26> nodes;
  TrieNode() : nodes(array<TrieNode *, 26>{nullptr}), isEnd(false) {}
};

class Trie {
 private:
  TrieNode *root;

 public:
  Trie() : root(new TrieNode()) {}

  void insert(string word) {
    auto r = this->root;
    for (const auto s : word) {
      if (r->nodes[static_cast<size_t>(s - 'a')] == nullptr) {
        r->nodes[static_cast<size_t>(s - 'a')] = new TrieNode();
      }
      r = r->nodes[static_cast<size_t>(s - 'a')];
    }
    r->isEnd = true;
  }

  bool search(string word) {
    auto r = this->root;
    for (const auto s : word) {
      if (r->nodes[static_cast<size_t>(s - 'a')] == nullptr) {
        return false;
      }
      r = r->nodes[static_cast<size_t>(s - 'a')];
    }
    return r->isEnd;
  }

  bool startsWith(string prefix) {
    auto r = this->root;
    for (const auto s : prefix) {
      if (r->nodes[static_cast<size_t>(s - 'a')] == nullptr) {
        return false;
      }
      r = r->nodes[static_cast<size_t>(s - 'a')];
    }
    return true;
  }
};

int main() { return 0; }
