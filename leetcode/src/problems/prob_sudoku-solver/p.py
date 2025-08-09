from typing import *
import os
import heapq
import copy


class Group:
    def __init__(self, s, x, y, box=False):
        self.s = s
        if box:
            self.positions = set(
                (x * 3 + _x, y * 3 + _y) for _x in range(3) for _y in range(3)
            )
        elif x is not None:
            self.positions = set((x, _y) for _y in range(9))
        elif y is not None:
            self.positions = set((_x, y) for _x in range(9))

    def unsetat(self, x, y, k):
        self.s.add(k)
        self.positions.add((x, y))

    def setat(self, x, y, k):
        self.s.remove(k)
        self.positions.remove((x, y))

    def checkat(self, x, y, k):
        return k in self.s and (x, y) in self.positions

    def __eq__(self, value) -> bool:
        return len(self.s) == len(value.s)

    def __lt__(self, other) -> bool:
        if len(self.s) == 0 and len(other.s) > 0:
            return False
        if len(other.s) == 0 and len(self.s) > 0:
            return True
        return len(self.s) < len(other.s)


class Solution:
    def solveSudoku(self, board: List[List[str]]) -> None:
        rows = [Group(set([i for i in range(1, 10)]), None, y) for y in range(9)]
        cols = [Group(set([i for i in range(1, 10)]), x, None) for x in range(9)]
        boxes = [
            Group(set([i for i in range(1, 10)]), k % 3, k // 3, box=True)
            for k in range(9)
        ]
        groups = rows + cols + boxes

        def read(x, y):
            if board[y][x] == ".":
                return None
            return int(board[y][x])

        def unsetat(x, y, k):
            rows[y].unsetat(x, y, k)
            cols[x].unsetat(x, y, k)
            boxes[(y // 3) * 3 + x // 3].unsetat(x, y, k)
            board[y][x] = "."

        def setat(x, y, k):
            rows[y].setat(x, y, k)
            cols[x].setat(x, y, k)
            boxes[(y // 3) * 3 + x // 3].setat(x, y, k)
            board[y][x] = str(k)

        def checkat(x, y, k):
            return (
                rows[y].checkat(x, y, k)
                and cols[x].checkat(x, y, k)
                and boxes[(y // 3) * 3 + x // 3].checkat(x, y, k)
            )

        for y in range(9):
            for x in range(9):
                k = read(x, y)
                if k:
                    setat(x, y, k)

        def do():
            heapq.heapify(groups)
            g = heapq.heappop(groups)
            heapq.heappush(groups, g)
            if len(g.s) == 0:
                return True
            k = g.s.pop()
            g.s.add(k)
            # pos = copy.copy(g.positions)
            for x, y in g.positions:
                if checkat(x, y, k):
                    setat(x, y, k)
                    if do():
                        return True
                    unsetat(x, y, k)
            return False

        do()


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    mat = [
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
        [".", "9", ".", ".", "1", ".", ".", "3", "."],
        [".", ".", "6", ".", "2", ".", "7", ".", "."],
        [".", ".", ".", "3", ".", "4", ".", ".", "."],
        ["2", "1", ".", ".", ".", ".", ".", "9", "8"],
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
        [".", ".", "2", "5", ".", "6", "4", ".", "."],
        [".", "8", ".", ".", ".", ".", ".", "1", "."],
        [".", ".", ".", ".", ".", ".", ".", ".", "."],
    ]
    sol.solveSudoku(mat)
    print(mat)
