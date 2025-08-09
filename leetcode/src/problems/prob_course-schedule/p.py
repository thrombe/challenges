from typing import *
import os


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        fwd = {i: set() for i in range(numCourses)}
        back = {i: set() for i in range(numCourses)}
        for a, b in prerequisites:
            back[b].add(a)
            fwd[a].add(b)

        stack = []
        for i in range(numCourses):
            if len(fwd[i]) == 0:
                stack.append(i)

        visited = set()
        while len(stack) > 0:
            i = stack.pop()
            visited.add(i)
            for j in back[i]:
                if i not in fwd[j]:
                    continue
                fwd[j].remove(i)
                if len(fwd[j]) == 0:
                    stack.append(j)

        return max(map(lambda x: len(x), fwd.values())) == 0


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
