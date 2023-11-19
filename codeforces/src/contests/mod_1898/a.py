for _ in range(int(input())):
    n, k = map(int, input().split())
    s = input()

    count = 0
    where = []
    for i, c in enumerate(s):
        if c == 'B':
            count += 1
            where.append(i)

    count = s.count('B')
    if k == count:
        print(0)
    elif k > count:
        b = k - count
        if b 
    elif k < count:
        pass
