for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    s = 0
    last_len = len(a)
    while len(a) > 0:
        print(a)
        last = 0
        d = a[0]
        isum = 0
        for i, e in enumerate(a):
            if d is not None and d > e:
                isum += d - e
            if e > last:
                a = a[i:]
                s += isum
                break
            last = max(e, last)
        # last = 0
        # d = a[-1]
        # isum = 0
        # for i, e in list(enumerate(a))[::-1]:
        #     if d is not None and d > e:
        #         isum += d - e
        #     if last > e:
        #         a = a[:i]
        #         s += isum
        #         break
        #     last = max(e, last)
        if last_len == len(a):
            break
        last_len = len(a)
    print(s)

    
    
