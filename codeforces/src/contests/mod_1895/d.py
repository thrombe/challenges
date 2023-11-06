import math

n = int(input())
a = list(map(int, input().split()))

b = [0]
m = 0
for i in a:
    p = b[-1] ^ i
    if p > n-1:
        m = m&int("1" * int(math.log2(n-1)), 2)&p
    b.append(p)
    m = max(p, m)


# b[0] = m^(n-1)
b[0] = int("1" * int(math.log2(n-1)), 2)&m
print(b[0], end=" ")
for i in range(1, n):
    b[i] = b[i-1]^a[i-1]
    print(b[i], end=" ")
print()
