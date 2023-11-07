for _ in range(int(input())):
    n,m = map(int, input().split())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))
    
    inc = []
    start = 0
    for i in range(n - 1):
        if a[i] >= a[i+1]:
            if i > start:
                inc.append((start, i))
            start = i+1
    if n-1 > start:
        inc.append((start, n-1))
    print(inc, a)
