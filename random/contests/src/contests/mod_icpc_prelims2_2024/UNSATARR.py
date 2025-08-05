import copy

class Node:
    def __init__(self, sett):
        self.left = None
        self.right = None
        self.sett = copy.copy(sett) 

t = int(input())
for _ in range(t):
    n, k = map(int, input().split())
    n_set = set([i for i in range(1, n+1)])
    # trips = []
    for _ in range(k):
        l, r, m = map(int, input().split())
        n_set.remove(m)
    if len(n_set) == 0:
        print(-1)
    else:
        m = max(list(n_set))
        print(*[m for _ in range(n)])

    
