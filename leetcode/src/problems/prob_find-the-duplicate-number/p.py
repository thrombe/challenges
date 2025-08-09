from typing import *
import os


class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        slow = 0
        fast = 0
        while True:
            slow = nums[slow]
            fast = nums[nums[fast]]
            if slow == fast:
                break
        slow2 = 0
        while slow != slow2:
            slow = nums[slow]
            slow2 = nums[slow2]
        return slow


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.findDuplicate([1, 3, 4, 2, 2])
    print(ans)
