from typing import *
import os


class Solution:
    def maxDistance(self, position: List[int], m: int) -> int:
        position = sorted(position)

        def try_for(n):
            balls = m
            last = -float("inf")
            for b in position:
                if balls == 0:
                    break
                if b - last >= n:
                    last = b
                    balls -= 1
            return balls == 0

        left = 0
        right = max(position)
        while left < right:
            mid = (left + right) // 2
            if try_for(mid):
                left = mid + 1
            else:
                right = mid - 1

        while True:
            if try_for(left):
                return left
            left -= 1


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
