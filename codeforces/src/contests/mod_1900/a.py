for _ in range(int(input())):
    n = int(input())
    s = input()

    spaces = list(s.split("#"))
    # print(spaces)

    inf = False
    for space in spaces:
        s = len(space)
        if s > 2:
            inf = True

    waters = 0
    for space in spaces:
        s = len(space)
        if s <= 2:
            waters += s
        else:
            waters += (s+1)//2
    if inf:
        print(2)
    else:
        print(waters)
