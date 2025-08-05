from typing import *


class Solution:
    def calcEquation(
        self, equations: List[List[str]], values: List[float], queries: List[List[str]]
    ) -> List[float]:
        import copy
        import random

        _subs = {}
        defined_vars = set()
        for (a, b), v in zip(equations, values):
            defined_vars.add(a)
            defined_vars.add(b)

            if a not in _subs:
                _subs[a] = []
            if b not in _subs:
                _subs[b] = []
            _subs[a].append([v, b])
            _subs[b].append([1 / v, a])

        ans = []

        def _solve_query(a, b, subs):
            if a not in defined_vars:
                ans.append(-1)
                return
            if b not in defined_vars:
                ans.append(-1)
                return

            if a == b:
                ans.append(1)
                return

            stack = [[1, a]]
            explored = set()
            while True:
                if len(stack) == 0:
                    ans.append(-1)
                    break
                val, var = stack.pop()
                if var == b:
                    ans.append(val)
                    break

                sub = subs[var]
                for v, s in sub:
                    edge = (val, v, var, s)
                    if edge not in explored:
                        explored.add(edge)
                        stack.append([val * v, s])

        for [a, b] in queries:
            subs = copy.deepcopy(_subs)
            _solve_query(a, b, subs)

        return ans


if __name__ == "__main__":
    sol = Solution()
    ans = sol.calcEquation(
        [["x1", "x2"], ["x2", "x3"], ["x3", "x4"], ["x4", "x5"]],
        [3.0, 4.0, 5.0, 6.0],
        [
            ["x1", "x5"],
            ["x5", "x2"],
            ["x2", "x4"],
            ["x2", "x2"],
            ["x2", "x9"],
            ["x9", "x9"],
        ],
    )

    print(ans)
