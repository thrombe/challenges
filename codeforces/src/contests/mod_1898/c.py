for _ in range(int(input())):
    n, m, k = map(int, input().split())

    mp = m+n-2
    if (k-mp)%2 == 1 or k < mp:
        print("NO")
        continue

    print("YES")
    h = [['R']*(m - 1) for _ in range(n)]
    v = [['B']*(m) for _ in range(n - 1)]
    v[0][0] = 'R'

    if n > m:
        for i in range(m-1, n-1, 2):
            v[i][m-1] = 'R'
    elif n < m:
        for i in range(n, m-1, 2):
            h[n-1][i] = 'B'
    else:
        pass

    for r in h:
        print(*r)
    for r in v:
        print(*r)

