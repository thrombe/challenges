from typing import *


def binaryQueries(n: int, a: List[int], q: int, queries: List[List[int]]) -> List[int]:
    ans = [0] * len(queries)

    xor_cumulative = [0] * (n + 1)
    for i in range(n):
        xor_cumulative[i + 1] = xor_cumulative[i] ^ a[i]

    for i, d in enumerate(queries):
        l, r, x = d

        xor_value = xor_cumulative[r + 1] ^ xor_cumulative[l] ^ x
        ans[i] = xor_value

    return ans


print(binaryQueries(4, [1, 2, 10, 6], 0, [[1, 3, 3], [0, 2, 6]]))



# std::vector<int> binaryQueries(int n, const std::vector<int>& a, int q, const std::vector<std::vector<int>>& queries) {
#     std::vector<int> ans(queries.size(), 0);

#     std::vector<int> xorCumulative(n + 1, 0);
#     for (int i = 0; i < n; ++i) {
#         xorCumulative[i + 1] = xorCumulative[i] ^ a[i];
#     }

#     for (int i = 0; i < queries.size(); ++i) {
#         int l = queries[i][0];
#         int r = queries[i][1];
#         int x = queries[i][2];

#         int xorValue = xorCumulative[r + 1] ^ xorCumulative[l] ^ x;
#         ans[i] = xorValue;
#     }

#     return ans;
# }