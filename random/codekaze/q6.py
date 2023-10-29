
from typing import *


def falseMemories(n: int, m: int, a: List[int]) -> int:
    return sum([1 if i%m != 0 else 0 for i in a])


print(falseMemories(0, 2, [9, 12, 11, 21]))
