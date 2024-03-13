for _ in range(int(input())):
    n = int(input())
    s = list(input().lower())

    children = []
    parent = [(-1, 'h')]*n
    leaves = []
    dp = [float('inf')]*n
    for i in range(n):
        el, ar = map(int, input().split())
        el -= 1
        ar -= 1

        children.append((el, ar))
        if max(el, ar) == -1:
            leaves.append(i)
            dp[i] = 0
        else:
            if el > -1:
                parent[el] = (i, 'l')
            if ar > -1:
                parent[ar] = (i, 'r')

    # print(children)
    # print(leaves)
    # print(parent)

    traversing = leaves.copy()
    next = set()
    while len(traversing) > 0:
        if dp[0] == 0:
            break
        # print(traversing, s)
        # print(next, "before loop")
        for i in traversing:
            p, d = parent[i]
            if p == -1:
                continue
            next.add(p)
            if d == s[p]:
                dp[p] = min(dp[p], dp[i])
            else:
                dp[p] = min(dp[p], dp[i] + 1)
        # print(next, "after loop")

        traversing = next
        next = set()
        # import time
        # time.sleep(2)

    # print(dp)
    print(dp[0])

