from typing import *
import os


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        ans = 0
        for i in range(len(heights)):
            h = heights[i]

            left = i
            for j in range(i, -1, -1):
                left = j
                if heights[j] < h:
                    left = j + 1
                    break
            right = i
            for j in range(i, len(heights)):
                right = j
                if heights[j] < h:
                    right = j - 1
                    break

            ans = max(ans, (right - left + 1) * h)

        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
