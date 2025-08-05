from typing import *
import os

if os.environ.get("SOLVING_LOCALLY"):
    # Definition for singly-linked list.
    class ListNode:
        def __init__(self, val=0, next=None):
            self.val = val
            self.next = next


class Solution:
    def rotateRight(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        if not head:
            return None
        n = 0
        last = head
        node = head
        while node is not None:
            last = node
            node = node.next
            n += 1
        last.next = head
        for _ in range((n + n - k) % n):
            last = last.next

        node = last.next
        last.next = None
        return node


if os.environ.get("SOLVING_LOCALLY"):
    sol = Solution()
    ans = sol.call()
    print(ans)
