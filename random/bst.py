

class Node:
    def __init__(self, num, left, right):
        self.left = left
        self.right = right
        self.data = num

    def create(liss):
        if len(liss) == 1:
            return Node(liss[0], None, None)
        elif len(liss) == 0: return None

        self_index = int(len(liss)/2)
        left = Node.create(liss[:self_index])
        right = Node.create(liss[self_index+1:])
        root = Node(liss[self_index], left, right)
        return root

    def in_order(self):
        if self.left is None: return [self.data]
        liss = []
        liss += self.left.in_order()
        liss.append(self.data)
        if self.right is None: return liss
        liss += self.right.in_order()
        return liss

    def max_depth(self):
        if self.left is None and self.right is None:
            return 0
        elif self.left is None or self.right is None:
            return 1
        return max(self.left.max_depth(), self.right.max_depth()) +1

a = Node.create([1, 2, 3, 4, 5, 6, 7, 8])
print(a.in_order())
print(a.max_depth())
