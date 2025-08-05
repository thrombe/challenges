from typing import *
import os
import heapq


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.val = x * x + y * y
        self.val = -self.val

    def __eq__(self, other):
        return self.val == other.val

    def __lt__(self, other):
        return self.val < other.val


class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        heap = []
        for [x, y] in points[:k]:
            heapq.heappush(heap, Point(x, y))

        for [x, y] in points[k:]:
            _ = heapq.heappushpop(heap, Point(x, y))

        return [[p.x, p.y] for p in heap]


if os.environ.get("SOLVING_LOCALLY"):
    pass
