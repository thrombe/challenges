t = int(input())
for _ in range(t):
    n, k = map(int, input().split())
    c = list(map(int, input().split()))

    c.sort()

    ps = [0]*n
    ps[0] = c[0]
    for i in range(1, n):
        ps[i] = c[i] + ps[i-1]

    dp = [0]*n

    for m in range(0, n): 
        # blocks = 0
        cost = 0
        # broke = False
        if (k+1) <= m+1:
            cost += ps[m] - ps[m - k]
            # blocks += 1
        if m < k:
            cost += ps[m%(k+1)]
        else:
            cost += dp[m - (k+0)]
        dp[m] = cost
        print(cost, end=" ")
    print()
    # cost = 0
    # for i in range(n):
    #     print(f"- {i} {cost} + {c[i]}")
    #     cost += c[i]
    #     if (i+1) % (k+1) == 0:
    #         print(f"- {i} {cost} - {c[i-k]}")
    #         cost -= c[i-k]
    #     # print(cost, end=" ")
    # print()
