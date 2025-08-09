from typing import *
import os


class Solution:
    def pacificAtlantic(self, heights: List[List[int]]) -> List[List[int]]:
        y = len(heights)
        x = len(heights[0])
        ocean1 = [[False] * x for _ in range(y)]
        ocean2 = [[False] * x for _ in range(y)]
        for _x in range(x):
            ocean1[0][_x] = True
            ocean2[-1][_x] = True
        for _y in range(y):
            ocean1[_y][0] = True
            ocean2[_y][-1] = True

        visited = [[False] * x for _ in range(y)]

        def do(_x, _y, ocean):
            a = _y
            b = _x
            if b >= x or b < 0 or a >= y or a < 0:
                return
            if ocean[_y][_x] or visited[_y][_x]:
                return
            visited[_y][_x] = True
            a = _y - 1
            b = _x
            if not (b >= x or b < 0 or a >= y or a < 0):
                if heights[_y][_x] >= heights[a][b]:
                    do(b, a, ocean)
                    ocean[_y][_x] = ocean[a][b] or ocean[_y][_x]
            a = _y
            b = _x - 1
            if not (b >= x or b < 0 or a >= y or a < 0):
                if heights[_y][_x] >= heights[a][b]:
                    do(b, a, ocean)
                    ocean[_y][_x] = ocean[a][b] or ocean[_y][_x]
            a = _y + 1
            b = _x
            if not (b >= x or b < 0 or a >= y or a < 0):
                if heights[_y][_x] >= heights[a][b]:
                    do(b, a, ocean)
                    ocean[_y][_x] = ocean[a][b] or ocean[_y][_x]
            a = _y
            b = _x + 1
            if not (b >= x or b < 0 or a >= y or a < 0):
                if heights[_y][_x] >= heights[a][b]:
                    do(b, a, ocean)
                    ocean[_y][_x] = ocean[a][b] or ocean[_y][_x]
            visited[_y][_x] = False

        for _y in range(y):
            for _x in range(x):
                do(_x, _y, ocean1)
                do(_x, _y, ocean2)

        ans = []
        for _y in range(y):
            for _x in range(x):
                if ocean1[_y][_x] and ocean2[_y][_x]:
                    ans.append([_y, _x])
        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.pacificAtlantic(
        [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ]
    )
    print(ans)
