
t = int(input())
for i in range(t):
    n, k = map(int, input().split())
    s = input()

    i = 0
    count = 0
    while i < len(s):
        if s[i] == 'W':
            i += 1
        else:
            count += 1
            i += k
    print(count)

