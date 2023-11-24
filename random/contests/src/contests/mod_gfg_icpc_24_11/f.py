a = list(map(int, input().split()))
a.sort(key=lambda x: -x)

n = int(input())

def rec(s, a):
    for i, e in enumerate(a):
        s += e
        if s == n:
            return 1
        k = rec(s, a)
        s -= e


