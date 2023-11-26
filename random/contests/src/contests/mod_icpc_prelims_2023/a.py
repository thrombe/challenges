def recurse(n, b, w, x, strike, memo):
    if x >= 100 and n <= 0:
        return 1
    if n <= 0 or b < 0 or w >= 10:
        return 0
    # if (b, w, x, strike) in memo:
    #     return memo[(b, w, x, strike)]

    if (b % 6 == 0):
        nstrike = not strike
    else:
        nstrike = strike
    b -= 1

    ans = 0
    if strike:
        if x >= 100:
            ans += recurse(n, b, w + 1, x, nstrike, memo)
        ans += recurse(n - 1, b, w, x + 1, not nstrike, memo)
        ans += recurse(n - 2, b, w, x + 2, nstrike, memo)
        ans += recurse(n - 3, b, w, x + 3, not nstrike, memo)
        ans += recurse(n - 4, b, w, x + 4, nstrike, memo)
        ans += recurse(n - 6, b, w, x + 6, nstrike, memo)
    else:
        ans += recurse(n, b, w + 1, x, nstrike, memo)
        ans += recurse(n - 1, b, w, x, not nstrike, memo)
        ans += recurse(n - 2, b, w, x, nstrike, memo)
        ans += recurse(n - 3, b, w, x, not nstrike, memo)
        ans += recurse(n - 4, b, w, x, nstrike, memo)
        ans += recurse(n - 6, b, w, x, nstrike, memo)

    ans %= 1000000007
    # memo[(b, w, x, strike)] = ans
    return ans


for _ in range(int(input())):
    n, b, w, x = map(int, input().split())
    memo = {}
    p = recurse(n, b-1, w, x, True, memo)
    q = pow(6, b, 1000000007)
    print((p * pow(q, -1, 1000000007)) % 1000000007)
