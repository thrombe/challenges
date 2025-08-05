from typing import *
import os


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
