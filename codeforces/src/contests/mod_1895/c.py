n = int(input())
ss = list(input().split())

pss = []
odds = []
evens = []
for (j, s) in enumerate(ss):
    ps = [0]
    for (i, c) in enumerate(s):
        ps.append(ps[i]+int(c))
    pss.append(ps)
    if len(s) % 2 == 0:
        evens.append(j)
    else:
        odds.append(j)

# print(pss)
ans = 0
for i in odds:
    for j in odds:
        m = (len(ss[i]) + len(ss[j]))//2
        if len(ss[i]) > len(ss[j]):
            lh = pss[i][m]
            rh = pss[j][-1] + pss[i][-1] - pss[i][m]
            if lh == rh:
                ans += 1
        elif len(ss[i]) < len(ss[j]):
            lh = pss[i][-1] + pss[j][m - len(pss[i]) + 1]
            rh = pss[j][-1] - pss[j][m - len(pss[i]) + 1]
            if lh == rh:
                ans += 1
        else:
            if pss[i][-1] == pss[j][-1]:
                ans += 1

for i in evens:
    for j in evens:
        m = (len(ss[i]) + len(ss[j]))//2
        if len(ss[i]) > len(ss[j]):
            lh = pss[i][m]
            rh = pss[j][-1] + pss[i][-1] - pss[i][m]
            if lh == rh:
                ans += 1
        elif len(ss[i]) < len(ss[j]):
            lh = pss[i][-1] + pss[j][m - len(pss[i]) + 1]
            rh = pss[j][-1] - pss[j][m - len(pss[i]) + 1]
            if lh == rh:
                ans += 1
        else:
            if pss[i][-1] == pss[j][-1]:
                ans += 1
print(ans)
