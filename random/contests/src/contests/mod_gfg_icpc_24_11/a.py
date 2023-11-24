for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))[:n]

    last = -float('inf')
    leaders = []
    while len(a) > 0:
        e = a.pop()
        if e >= last:
            leaders.append(e)
            last = e 
    while len(leaders) > 0:
        e = leaders.pop()
        print(e, end=' ')
    print()
            
