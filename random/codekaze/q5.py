
from typing import *


def maxiAnd(n: int, a: List[int]) -> int:
    m = min(a)
    p = 0
    while m > 0:
        m = m//2
        p += 1
    ans = 0
    for i in range(p):
        ans = ans << 1
        ans = ans | 1
    return ans


print(maxiAnd(0, [15, 25, 25, 30, 20]))
        