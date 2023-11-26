
def bsearch(arr, e):
    p = 0
    r = len(arr) - 1
    while p <= r:
        mid = p + (r - p) // 2
        if arr[mid] == e:
            k = 0
            kk = 0
            for i in range(mid - 1, -1, -1):
                if arr[i] == e:
                    k += 1
                else:
                    break
            for i in range(mid+1, len(arr)):
                if arr[i] == e:
                    kk += 1
                else:
                    break
            return True, mid - k, mid + kk # both inclusive
        elif arr[mid] < e:
            p = mid + 1
        else:
            r = mid - 1
    return False, p, p

# u = [ 1, 2, 2, 2, 4, 4]
# print(bsearch(u, 2))

def score(a, b):
    c = 0
    for i, j in zip(a, b):
        if i > j:
            c += 1
    return c

for _ in range(int(input())):
    # print()
    # print()
    n, x = map(int, input().split())
    a = list(map(int, input().split()))
    ao = a.copy()
    b = list(map(int, input().split()))

    a.sort()
    b.sort()
    ac = [-1]*n

    for k, e in enumerate(a):
        p, i, j = bsearch(b, e)
        ac[k] = i
    # print("ac", ac)
    # print(x)
    # print(a, b)

    if ac[-1] < x:
        print("NO")
        continue

    # print("assssssssssssssss")
    # print(ac)
    ac = ac[:n - (ac[-1] - x)]
    # print(ac)

    if len(ac) == 0:
        p = x
    else:
        p = ac[-1] - x
    # swaps = []
    # print("score", score(a, b), x)
    # for i, (k1, k2) in enumerate(zip(ac[:len(ac)//2], ac[::-1])):
    #     if k2 > k1:
    #         swaps.append(i)
    #         b[i], b[len(ac)-i-1] = b[len(ac)-i-1], b[i]
    #         p -= 1
    #     else:
    #         break
    o = (len(ac)+1)//2
    for i, k1 in enumerate(ac[:len(ac)//2]):
        for j in range(o, len(ac)):
            k2 = ac[j]
            if k2 > k1:
                # swaps.append(i)
                b[i], b[j] = b[j], b[i]
                p -= 1

                ac[j] = -1
                o += j+1
                break
            if len(ac)-1 == j:
                o = len(ac)
    # print(ac)
            

    if p > 0:
        print("NO")
        continue

    # print(a, b)
    # print("swaps", swaps)

    print("YES")

    indices = {}
    for i, e in enumerate(ao):
        if e in indices:
            indices[e].append(i)
        else:
            indices[e] = [i]

    ans = [-1]*n
    for i, e in enumerate(a):
        j = indices[e].pop()
        ans[j] = b[i]

    # print("score", score(ao, ans), x)

    # print(ao, ans)
    print(*ans)

    # k = 0
    # for e in b:
    #     p, i, j = bsearch(a, e)
    #     if j == 0:
    #         a.pop(0)
    #         continue
    #     elif i >= len(a) - 1:
    #         continue

    #     if p:
    #         a.pop(j+1)
    #     else:
    #         if a[i] == e:
    #             a.pop(i+1)
    #         else:
    #             a.pop(i)

    # print(a)
    # if k < x:
    #     print("NO")
    #     continue
    # print("YES")

    
