def bsearch(arr, e):
    p = 0
    r = len(arr) - 1
    while p <= r:
        mid = p + (r - p) // 2
        if arr[mid] == e:
            return mid
        elif arr[mid] < e:
            p = mid + 1
        else:
            r = mid - 1
    return p

# k = bsearch([1, 2, 3, 6, 8, 9], 10)
# print(k)


for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))
    # b.sort()
    a.sort()

    counts = {}
    for e in a:
        if e in counts:
            counts[e] += 1
        else:
            counts[e] = 1

    p = max(counts.values())
    if p > (n+1) // 2:
    # if (p%2 == 0 and p > n//2) or (p%2 == 1 and p > n//2 + 1):
        print('NO')
        continue

    found = False
    for e in b:
        k = bsearch(a, e)
        if k == 0 and a[k] > e:
            found = True
            break
        if len(a) == k:
            a.pop(k-1)
            continue
        else:
            if a[k] > e:
                a.pop(k-1)
            else:
                a.pop(k)

    if found:
        print("NO")
    else:
        print("YES")
	
	
	
	
