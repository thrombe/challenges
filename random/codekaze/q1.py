
from typing import *


def contributions(n: int, a: List[int]) -> int:
    mod3 = {}
    for i, b in enumerate(a):
        m = b%3
        if mod3.get(m, None) is None:
            mod3[m] = [i]
        else:
            mod3[m].append(i)
    s = 0
    for nums in mod3.values():
        for i, x in enumerate(nums):
            for y in nums[i+1:]:
                s += a[x]^a[y]
    return s



print(contributions(0, [1, 2, 3, 4, 5, 6, 7, 8]))
    