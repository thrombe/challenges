for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    a.sort()
    xs = a[:n]
    ys = a[n:]
    # print(xs, ys)

    path = []
    s = 0
    for i in range(n):
        point = (xs[i], ys[i])
        path.append(point)
        if len(path) > 1:
            s += abs(path[-1][0] - path[-2][0]) + abs(path[-1][1] - path[-2][1])
    print(s)
    for (x, y) in path:
        print(x, y)
        
