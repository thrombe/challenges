from typing import *
import os


class Solution:
    def decodeString(self, s: str) -> str:
        def do_brac(i):
            subans = []
            while i < len(s):
                c = s[i]
                if c == "]":
                    return i + 1, subans
                elif c >= "0" and c <= "9":
                    n = 0
                    while s[i] >= "0" and s[i] <= "9":
                        c = s[i]
                        d = ord(c) - ord("0")
                        n *= 10
                        n += d
                        i += 1
                    j, subsubans = do_brac(i + 1)
                    for _ in range(n):
                        subans.extend(subsubans)
                    i = j
                else:
                    subans.append(c)
                    i += 1
            return i, subans

        _, ans = do_brac(0)
        return "".join(ans)


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
