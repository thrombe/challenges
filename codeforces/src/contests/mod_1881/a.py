
for _ in range(int(input())):
    n, m = map(int, input().split())
    x = input()
    s = input()

    num = 0
    while len(x) < len(s):
        num += 1
        x += x

    if s not in x:
        num += 1
        x += x
    if s not in x:
        print(-1)
    else:
        print(num)
