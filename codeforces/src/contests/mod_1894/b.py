for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    indices = {}
    for (i, e) in enumerate(a):
        if e not in indices:
            indices[e] = [i]
        else:
            indices[e].append(i)

    ans = [1]*n
    onedone = False
    twodone = False
    for v in indices.values():
        if not onedone:
            if len(v) > 1:
                ans[v[1]] = 2
                onedone = True
        else:
            if len(v) > 1:
                ans[v[1]] = 3
                twodone = True
                break
    if twodone:
        print(*ans)
    else:
        print(-1)

 
