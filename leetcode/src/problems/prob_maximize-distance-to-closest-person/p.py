from typing import *
import os


class Solution:
    # suboptimal solution :/
    # O(n) solution is pretty easy here :P
    def maxDistToClosest(self, seats: List[int]) -> int:
        n = len(seats)

        positions = [-1]
        for i, t in enumerate(seats):
            if t == 1:
                positions.append(i)

        def try_for(k):
            positions[0] = -k
            positions.append(n + k - 1)
            for a, b in zip(positions, positions[1:]):
                if b - a - 1 >= 2 * k - 1:
                    positions.pop()
                    return True
            positions.pop()
            return False

        left = 0
        right = n
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
