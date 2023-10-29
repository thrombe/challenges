for _ in range(int(input())):
    n = int(input())
    a = map(int, input().split())

    counts = {}
    for i in a:
        if i in counts:
            counts[i] += 1
        else:
            counts[i] = 1

    if len(counts) > 2:
        print("no")
        continue

    items = list(counts.items())
    if abs(items[0][1] - items[-1][1]) < 2:
        print("yes")
    else:
        print("no")
