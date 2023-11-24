n = input()

counts = {}

for c in n:
    if c in counts:
        counts[c] += 1
    else:
        counts[c] = 1

a = list(counts.items())
a.sort(key=lambda x: x[1] + 1/(1 +int(x[0])))

print(a[-1][0])
    
