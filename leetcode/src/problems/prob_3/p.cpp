
#include <bits/stdc++.h>
using namespace std;

class Solution {
  int arr[256];

public:
  int lengthOfLongestSubstring(string s) {
    for (int i = 0; i < 256; i++) {
      arr[i] = -1;
    }

    int i = 0, j = 0;
    int maxlen = 0;
    while (s.size() > j) {
      char c = s[j];
      if (arr[c] != -1) {
        // move i to find the last found char
        char k = s[i];
        while (k != c) {
          arr[k] = -1;
          i += 1;
          k = s[i];
        }
        i += 1;
        arr[c] = j;
        j += 1;
      } else {
        arr[c] = j;
        j += 1;
        maxlen = max(maxlen, j - i);
      }
    }
    return maxlen;
  }
};

int main() {
  cin.tie(0)->sync_with_stdio(0);
  Solution sol;

  string s;
  cin >> s;
  auto ans = sol.lengthOfLongestSubstring(s);
  cout << ans << endl;
}
