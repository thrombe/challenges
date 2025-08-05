from typing import *
import os


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        ps = {0: 1}
        ans = 0
        s = 0
        for n in nums:
            s += n
            if s - k in ps:
                ans += ps[s - k]
            if s not in ps:
                ps[s] = 0
            ps[s] += 1
        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.subarraySum([1, 1, 1], 2)
    print(ans)
