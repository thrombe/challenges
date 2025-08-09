from typing import *
import os


class Solution:
    def distributeCandies(self, n: int, limit: int) -> int:
        ans = 0
        for i in range(min(n, limit) + 1):
            ans += max(min(limit, n - i) - max(0, n - i - limit) + 1, 0)
        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.distributeCandies(100, 2)
    print(ans)
