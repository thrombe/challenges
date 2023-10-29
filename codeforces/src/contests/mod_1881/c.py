
def abs(a):
    if a >=0:
        return a
    else:
        return -a

for _ in range(int(input())):
    n = int(input())
    m = list(map(lambda _: input(), range(n)))

    k = n-1
    count = 0
    for i in range((n+1)//2):
        for j in range((n+1)//2):
            indices = [(i, j), (j, k-i), (k-i, k-j), (k-j, i)]
            mx = 0
            for (y, x) in indices:
                mx = max(mx, ord(m[y][x]))
            for (y, x) in indices:
                count += mx - ord(m[y][x])
    print(count)
