import math

for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    m = int(math.log2(n))
    curr_pow = 0
    found = False
    for _ in range(m):
        if found:
            break
        s = a[2**(curr_pow):2**(curr_pow + 1)]
        # print(s)
        for (i, j) in zip(s, s[1:]):
            if i > j:
                found = True
                break
        curr_pow += 1
    s = a[2**(curr_pow):]
    for (i, j) in zip(s, s[1:]):
        if i > j:
            found = True
            break
    if found:
        print("NO")
    else:
        print("YES")
