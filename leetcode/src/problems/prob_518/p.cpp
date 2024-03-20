
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int change(int a, vector<int> &coins) {
    for (int c : coins) {
      cout << c << " ";
    }
    cout << "lmao" << endl;
    vector<int> dp(a, -2);
    int k = rec(coins, a, dp, 0);
    for (int c : dp) {
      cout << c << " ";
    }
    cout << "lmao" << endl;
    return k;
  }
  int rec(vector<int> &coins, int t, vector<int> &dp, int m) {
    if (t == 0) {
      return 1;
    }
    if (t < 0) {
      return 0;
    }
    if (dp[t - 1] != -2) {
      return dp[t - 1];
    }
    int c = 0;
    for (int i = 0; i < coins.size(); i++) {
      if (coins[i] < m) {
        continue;
      }
      int k = rec(coins, t - coins[i], dp, coins[i]);
      cout << k << " " << t << " - " << coins[i] << " " << m << endl;
      c += k;
    }
    dp[t - 1] = c;
    return c;
  }
};

void solve() {
  cin.tie(0)->sync_with_stdio(0);
  Solution sol;

  int t;
  cin >> t;
  while (t--) {
    int n;
    cin >> n;
    vector<int> v(n, 0);
    for (int i = 0; i < n; i++) {
      cin >> v[i];
    }
    int a;
    cin >> a;
    auto ans = sol.change(a, v);
    cout << ans << endl;
  }
}
