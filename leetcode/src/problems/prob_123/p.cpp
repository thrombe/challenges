
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int maxProfit(vector<int> &prices) {
    int n = prices.size();
    vector<int> v(n, 0);
    // v[0] = prices[0];
    // v[n-1] = prices[n-1];
    int m = prices[0];
    int p = 0;
    for (int i = 1; i < n; i++) {
      p = max(prices[i] - m, p);
      v[i] = p;
      m = min(m, prices[i]);
    }
    cout << endl;
    m = prices[n - 1];
    p = 0;
    for (int i = n - 2; i >= 0; i--) {
      p = max(m - prices[i], p);
      v[i] += p;
      m = max(m, prices[i]);
    }
    m = 0;
    for (int i = 0; i < n; i++) {
      m = max(m, v[i]);
    }
    return m;
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
    vector<int> v(n);
    for (int i = 0; i < n; i++) {
      cin >> v[i];
    }
    auto ans = sol.maxProfit(v);
    cout << ans << endl;
  }
}
