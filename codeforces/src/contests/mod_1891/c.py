for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    a.sort()

    ps = [0]*n
    ps[0] += a[0]
    for i in range(1, n):
        ps = ps[i-1]+a[i]

    j = 0
    for i in ps:
        if a[i] 
    
    i = 0
    x = 0
    hits = 0
    while a[-1] > 0:
        if a[i] == 0:
            i += 1
            continue
        if a[i] == 1:
            hits += 1
            x += 1
