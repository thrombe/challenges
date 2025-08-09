from typing import *
import os


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        canreach = [False] * len(nums)
        canreach[0] = True
        for i, n in enumerate(nums):
            if not canreach[i]:
                continue
            for j in range(i + 1, min(i + n + 1, len(nums))):
                canreach[j] = True
            if canreach[-1]:
                return True
        return canreach[-1]


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.canJump([1, 2])
    print(ans)
