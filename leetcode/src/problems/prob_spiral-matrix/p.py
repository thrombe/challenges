from typing import *
import os

RIGHT = 0
DOWN = 1
LEFT = 2
UP = 3
COUNT = 4


class Solution:
    def spiralOrder(self, matrix: List[List[int]]) -> List[int]:
        state = RIGHT
        ly = len(matrix)
        lx = len(matrix[0])
        x = 0
        y = 0
        ans = []
        while x < lx and y < ly and matrix[y][x] is not None:
            c = matrix[y][x]
            ans.append(c)
            matrix[y][x] = None  # type: ignore
            if state == RIGHT:
                if x + 1 < lx and matrix[y][x + 1] is not None:
                    x += 1
                else:
                    state = DOWN
                    y += 1
            elif state == DOWN:
                if y + 1 < ly and matrix[y + 1][x] is not None:
                    y += 1
                else:
                    state = LEFT
                    x -= 1
            elif state == LEFT:
                if x - 1 >= 0 and matrix[y][x - 1] is not None:
                    x -= 1
                else:
                    state = UP
                    y -= 1
            elif state == UP:
                if y - 1 >= 0 and matrix[y - 1][x] is not None:
                    y -= 1
                else:
                    state = RIGHT
                    x += 1
        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
