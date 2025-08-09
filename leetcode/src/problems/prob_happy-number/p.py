from typing import *
import os


def ssq(n):
    ans = 0
    for d in str(n):
        d = ord(d) - ord("0")
        ans += d * d
    return ans


class Solution:
    def isHappy(self, n: int) -> bool:
        found = set()
        while True:
            if n == 1:
                return True
            if n in found:
                return False
            found.add(n)
            n = ssq(n)


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.isHappy(19)
    print(ans)
