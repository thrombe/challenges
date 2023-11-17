for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    # ps = [0]*n
    # for i, e in range(n):
    #     ps[i]

    ad = 0
    for k in range(1, n//2 + 1):
        if n%k != 0:
            continue

        m = -float('inf')
        mi = float('inf')

        for i in range(n//k):
            ar = a[i*k : k*(i+1)]
            # print(ar)
            s = sum(ar)
            m = max(m, s)
            mi = min(mi, s)
        
        ad = max(ad, abs(m - mi))
    print(ad)


    
