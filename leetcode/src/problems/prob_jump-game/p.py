from typing import *
import os


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        jump = nums[0]
        for n in nums[1:]:
            if jump <= 0:
                return False
            jump -= 1
            jump = max(jump, n)
        return True


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.canJump([3, 2, 1, 0, 4])
    print(ans)
