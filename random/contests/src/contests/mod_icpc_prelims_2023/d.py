for _ in range((int(input()))):
    n, k = map(int, input().split())
    s = input()

    c = 0
    clock = True
    for m in s:
        if m == 'U':
            if clock:
                c += 1
            else:
                c -= 1
        elif m == 'S':
            if clock:
                c += 2
            else:
                c -= 2
        elif m == 'R':
            clock = not clock
            if clock:
                c += 1
            else:
                c -= 1
    print((c % n) + 1)
