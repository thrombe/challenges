for _ in range(int(input())):
    n, m = map(int, input().split())
    s = input()
    t = input()

    if all(map(lambda x: x[0] != x[1], zip(s, s[1:]))):
        print("yes")
        continue
    if not all(map(lambda x: x[0] != x[1], zip(t, t[1:]))):
        print("no")
        continue

    i = 1
    last = s[0]
    found = False
    while i < len(s):
        new = s[i]
        if last == new:
            if last == t[0] or new == t[-1]:
                print("no")
                found = True
                break
        last = new
        i += 1

    if not found:
        print("yes")
