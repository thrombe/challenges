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
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        if not root:
            return True
        curr = None

        def do(node: TreeNode):
            nonlocal curr
            if node.left:
                if not do(node.left):
                    return False
            if curr is None:
                curr = node.val - 1
            if node.val <= curr:
                return False
            curr = node.val
            if node.right:
                if not do(node.right):
                    return False
            return True

        return do(root)


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
