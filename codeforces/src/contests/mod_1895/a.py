for _ in range(int(input())):
    x, y, k = map(int, input().split())
    if x < y:
        print(y + max(0, y-x - k))
    else:
        print(x)
