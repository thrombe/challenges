for _ in range(int(input())):
    n, k = map(int, input().split())
    a = list(map(int, input().split()))

    bids = [[] for _ in range(k)]
    a.sort()
    amin = a.copy()
    amin.sort(key=lambda x: -x)

    bmax = 0
    for i in range(k):
        b1 = a.pop()
        b2 = a.pop()

        bmax += b2

    # i = 0
    # while len(amin) > 0:
    #     bids[i].append(amin.pop())
    #     i += 1
    #     i %= k

    amin = amin[k:]
    # print(amin)

    bmin = 0
    extra = 0
    for i in range(k):
        b1 = amin.pop()
        if i == k-1:
            extra = b1
        bmin += b1
        # print(b1)

    # extra2 = 0
    # for i in range(k):
    #     b1 = amin.pop()
    #     if i == k-1:
    #         extra2 = b1

    # print(amin)
    if len(amin) > 0:
        bmin += amin[0]
        bmin -= extra
    # elif len(amin) == 1:
    #     bmin -= extra
    #     bmin += extra2
        

    # print(bids)

    print(bmin, bmax)

