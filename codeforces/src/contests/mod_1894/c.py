for _ in range(int(input())):
    n, k = map(int, input().split())
    b = list(map(int, input().split()))

    i = -1
    happened = set()
    for j in range(k):
        if i in happened:
            print("YES")
            break
        # if b[i] == n:
        #     print("YES")
        #     break
        elif b[i] > n:
            print("NO")
            break
        happened.add(i)
        i -= b[i]
        if i < -n:
            i += n
        if j == k-1:
            print("YES")
