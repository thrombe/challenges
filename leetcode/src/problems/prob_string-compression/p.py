from typing import *
import os


class Solution:
    def compress(self, chars: List[str]) -> int:
        read = 0
        write = 0
        curr = "\0"
        count = 0
        while read < len(chars):
            if chars[read] == curr:
                count += 1
            else:
                if count == 0:
                    pass
                elif count == 1:
                    chars[write] = curr
                    write += 1
                else:
                    chars[write] = curr
                    write += 1
                    num = str(count)
                    for c in num:
                        chars[write] = c
                        write += 1

                curr = chars[read]
                count = 1
            read += 1

        chars[write] = curr
        write += 1
        if count > 1:
            num = str(count)
            for c in num:
                chars[write] = c
                write += 1

        return write


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    strr = ["a", "a", "b", "b", "c", "c", "c"]
    ans = sol.compress(strr)
    print(strr[:ans])
