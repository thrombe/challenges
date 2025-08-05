
t = int(input())
for _ in range(t):
    n, k = map(int, input().split())
    a = list(map(int, input().split()))

    ans = ~0
    ans_list = []
    for i, ai in enumerate(a):
        print(ai, k)
        if ai & (~k) == 0:
            ans &= ai
            ans_list.append(i)
    if ans == k:
        print("YES")
        print(len(ans_list))
        print(*ans_list)
    else:
        print("NO")
            

