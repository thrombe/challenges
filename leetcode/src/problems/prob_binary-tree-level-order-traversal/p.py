from typing import *
import os

if os.environ.get("SOLVING_LOCALLY"):
    # Definition for a binary tree node.
    class TreeNode:
        def __init__(self, val=0, left=None, right=None):
            self.val = val
            self.left = left
            self.right = right


class Solution:
    def levelOrder(self, root: Optional[TreeNode]) -> List[List[int]]:
        levels = {}

        def traverse(node: Optional[TreeNode], depth=0):
            if not node:
                return
            traverse(node.left, depth + 1)
            traverse(node.right, depth + 1)
            if depth not in levels:
                levels[depth] = []
            levels[depth].append(node.val)

        traverse(root)

        ans = []
        i = 0
        while i < len(levels):
            ans.append(levels[i])
            i += 1

        return ans


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
