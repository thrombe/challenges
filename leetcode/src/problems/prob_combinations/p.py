from typing import *
import os
import copy


class Solution:
    def combine(self, n: int, k: int) -> List[List[int]]:
        ans = []

        def do(subans, i, depth=0):
            subans.append(i)
            if depth == k - 1:
                ans.append(subans)
                return

            for j in range(i + 1, n + 1):
                subans_j = copy.deepcopy(subans)
                do(subans_j, j, depth + 1)

        for i in range(1, n + 1):
            do([], i)
        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
