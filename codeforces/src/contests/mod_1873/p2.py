
t = int(input())
for i in range(t):
    n = int(input())
    a = list(map(int, input().split()))

    mult = 1
    m = 10
    zs = 0
    for e in a:
        if e != 0:
            mult *= e
        else:
            zs += 1
        m = min(m, e)

    if zs > 1:
        mult = 0
    elif zs == 0:
        mult = mult // m
        mult *= (m + 1)
        
    print(mult)
