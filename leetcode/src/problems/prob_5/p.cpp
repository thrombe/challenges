
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  string longestPalindrome(string s) {
    for (int i = 0; i < s.size(); i++) {
      pal(s, i);
    }
    return s.substr(start, end - start + 1);
  }

  int maxlen = 0;
  int start = 0;
  int end = 0;

  void pal(string s, int i) {
    // odd length
    int a = i, b = i;
    while (a >= 0 && b < s.size() && s[a] == s[b]) {
      a -= 1;
      b += 1;
    }
    a += 1;
    b -= 1;
    int len = b - a + 1;
    if (len > maxlen) {
      maxlen = len;
      start = a;
      end = b;
    }

    // even length
    a = i, b = i + 1;
    while (a >= 0 && b < s.size() && s[a] == s[b]) {
      a -= 1;
      b += 1;
    }
    a += 1;
    b -= 1;
    len = b - a + 1;
    if (len > maxlen) {
      maxlen = len;
      start = a;
      end = b;
    }
  }
};

void solve() {
  cin.tie(0)->sync_with_stdio(0);
  Solution sol;

  string s;
  cin >> s;
  auto ans = sol.longestPalindrome(s);
  cout << ans << endl;
}
