for _ in range(int(input())):
    n, m, k = map(int, input().split())
    if k == 1:
        print(1)
    elif k == 2:
        if m > n:
            print(n + m//n - 1)
        else:
            print(m)
    elif k == 3:
        if m > n:
            print(1+m - n - m//n)
        else:
            print(0)
    else:
        print(0)
