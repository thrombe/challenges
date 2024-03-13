def ans(a, b, c):
    ab = max(b, c)
    ac = min(b, c)
    ab -= ac
    ac = 0
    if ab%2 == 0:
        return 1
    else:
        return 0


for _ in range(int(input())):
    a, b, c = map(int, input().split())

    print(ans(a, b, c), ans(b, a, c), ans(c, a, b))

