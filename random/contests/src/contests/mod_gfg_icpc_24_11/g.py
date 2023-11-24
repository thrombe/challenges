for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    tot = 0
    while True:
        lowest = a[-1]
        highest = a[-1]
        for i, e in list(enumerate(a))[::-1]:
            if e >= highest:
                highest = e
            else:
                tot += highest - lowest
                a = a[:i]
                
