for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))
    a = a[:n]

    last = a.pop()
    ops = 0
    while len(a) > 0:
        if a[-1] > last:
            p = a.pop()
            # t = (p-1)//last
            # ops += t
            # last = p//(t+1)
            if p%last == 0:
                last = last
                ops += p//last - 1
            else:
                t = p//last
                ops += t
                last = p//(t + 1)
        else:
            last = a.pop()
    print(ops)
