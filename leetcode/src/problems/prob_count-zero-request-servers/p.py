from typing import *
import os


class Solution:
    def countServers(
        self, n: int, logs: List[List[int]], x: int, queries: List[int]
    ) -> List[int]:
        logs = sorted(logs, key=lambda x: x[1])
        ans_map = {}
        q_set = set(queries)
        sorted_queries = sorted(q_set)

        counts = {}
        i = 0
        j = 0
        for q in sorted_queries:
            while j < len(logs) and logs[j][1] <= q:
                server, time = logs[j]
                if server not in counts:
                    counts[server] = 0
                counts[server] += 1
                j += 1

            while i < len(logs) and logs[i][1] < q - x:
                server, time = logs[i]
                if server in counts:
                    counts[server] -= 1
                    if counts[server] == 0:
                        _ = counts.pop(server)
                i += 1

            ans_map[q] = n - len(counts)

        return [ans_map[q] for q in queries]


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.countServers(
        3,
        [[1, 3], [2, 6], [1, 5]],
        5,
        [10, 11],
    )
    print(ans)
