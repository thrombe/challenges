from typing import *
import os


class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        left = []
        stack = []
        for i in range(len(heights)):
            while len(stack) > 0 and heights[i] <= heights[stack[-1]]:
                stack.pop()

            if len(stack) > 0:
                left.append(stack[-1])
            else:
                left.append(-1)
            stack.append(i)

        right = []
        stack = []
        for i in range(len(heights)):
            j = len(heights) - i - 1
            while len(stack) > 0 and heights[j] <= heights[stack[-1]]:
                stack.pop()

            if len(stack) > 0:
                right.append(stack[-1])
            else:
                right.append(len(heights))
            stack.append(j)
        right = right[::-1]

        ans = 0
        for i in range(len(heights)):
            h = heights[i]
            ans = max(ans, (right[i] - left[i] - 1) * h)

        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
