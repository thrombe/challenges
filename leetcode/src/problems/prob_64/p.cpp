
#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
  int minPathSum(vector<vector<int>> &grid) {
    int m = grid.size();
    int n = grid[0].size();
    vector<vector<int>> dp(m, vector<int>(n, 0));
    dp[0][0] = grid[0][0];
    cout << dp[0][0] << " ";
    for (int y = 0; y < m; y++) {
      for (int x = 0; x < n; x++) {
        if (x == 0 && y == 0) {
          continue;
        }
        int c = grid[y][x];
        if (y == 0) {
          dp[y][x] = dp[y][x - 1] + c;
        } else if (x == 0) {
          dp[y][x] = dp[y - 1][x] + c;
        } else {
          dp[y][x] = min(dp[y - 1][x], dp[y][x - 1]) + c;
        }
        cout << dp[y][x] << " ";
      }
      cout << endl;
    }
    return dp[m - 1][n - 1];
  }
};

void solve() {
  cin.tie(0)->sync_with_stdio(0);
  Solution sol;

  int t;
  cin >> t;
  while (t--) {
    int y;
    int x;
    cin >> y >> x;
    cout << y << " " << x << " " << endl;
    vector<vector<int>> v(y, vector<int>(x, 0));
    for (int j = 0; j < y; j++) {
      for (int i = 0; i < x; i++) {
        cin >> v[j][i];
        cout << v[j][i] << " ";
      }
      cout << endl;
    }
    auto ans = sol.minPathSum(v);
    cout << ans << endl;
  }
}
