
for _ in range(int(input())):
    n, m = map(int, input().split())
    s = list(map(lambda x: x - ord('a'), map(ord, input())))
    if len(s) != n:
        n.pikaw

    def calc(i):
        if i >= n-1:
            return False
        elif i == n-2:
            return s[i] == s[i+1]
        return s[i] == s[i+2] or s[i] == s[i+1] or s[i+1] == s[i+2]

    precalc = [calc(i) for i in range(n)]
    # print(precalc)

    for _ in range(m):
        q = list(map(int, input().split()))
        # print(''.join(map(chr, map(lambda x: x + ord('a'), s))), precalc, q)
        if q[0] == 1:
            l, r, x = q[1:]
            for i in range(l-1, r):
                s[i] = (s[i] + x)%26
            for i in range(max(l-3, 0), r):
                precalc[i] = calc(i)
        elif q[0] == 2:
            l, r = q[1:]
            # print(''.join(map(chr, map(lambda x: x + ord('a'), s[l-1:r]))))
            # print(s, s[r-2], s[r-1], s[l-1:r-2])
            if r >= l+1 and any(precalc[l-1:r-2]):# or s[r-1] == s[r-2]:
                print("no")
            else:
                print("yes")
        else:
            n.pikaw

    T-T T-T T-T T-T T-T T-T T-T T-T T-T T-T T-T T-T
                
