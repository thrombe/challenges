


for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    dp = [float("inf")]*(n + 1)
    dp[n] = 0

    for i in range(n):
        j = n-i-1

        # remove
        dp[j] = 1 + dp[j+1]
        if j + a[j] < n:
            # check if removing is better or considering this as a block
            dp[j] = min(dp[j], dp[j + a[j] + 1])
    print(dp[0])

    
