
def ffs(x):
    """Returns the index, counting from 0, of the
    least significant set bit in `x`.
    """
    return (x&-x).bit_length()-1


for _ in range(int(input())):
    n, q = map(int, input().split())

    a = list(map(int, input().split()))
    x = list(map(int, input().split()))

    sets = [set() for _ in range(31)]
    lso = [ffs(i) for i in a]
    for i, p in enumerate(lso):
        sets[p].add(i)
    # print(sets)

    for p in x:
        for k in sets[p:]:
            for i in k:
                sets[p-1].add(i)
                a[i] += 2**(p-1)
            k.clear()

    for i in a:
        print(i, end=" ")
    print()
    
