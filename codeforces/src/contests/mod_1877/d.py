
n = int(input())
a = list(map(int, input().split()))

pm = [i for i in a]
for i in range(1, n):
    j = n - i - 1
    pm[j] = max(pm[j+1], a[j])

:skull


