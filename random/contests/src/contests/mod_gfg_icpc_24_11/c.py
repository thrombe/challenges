for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))[:n]

    indices = {}
    oddsum = 0
    evesum = 0
    for i, e in enumerate(a):
        if i%2 == 0:
            evesum += e
        else:
            oddsum += e
        if e in indices:
            if i%2 == 0:
                indices[e][0].append(i)
            else:
                indices[e][1].append(i)
        else:
            if i%2 == 0:
                indices[e] = ([i], [])
            else:
                indices[e] = ([], [i])

    b = a.copy()
    b.sort(key=lambda x: -x)

    ops = 0
    while len(b) > 0:
        # print(b)
        e = b[-1]
        if e == 0:
            b.pop()
        if oddsum > evesum:
            break
        found = False
        if len(indices[e][0]) > 0:
            ops += 1
            indices[e][0].pop()
            evesum -= e
            found = True
            continue
        if len(indices[e][1]) > 0:
            ops += 1
            indices[e][1].pop()
            oddsum -= e
            found = True
            continue

        if not found:
            b.pop()

    if oddsum > evesum:
        print('YES')
    else:
        print('NO')

        


    
