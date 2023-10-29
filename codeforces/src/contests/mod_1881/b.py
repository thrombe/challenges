
for _ in range(int(input())):
    a, b, c = sorted(list(map(int, input().split())))
    # print(a, b, c)
    
    if (a == b and a == c) or (a == b and 2*a == c) or (a == b and 3*a == c) or (2*a == b and 2*a == c) or (2*a == b and 3*a == c) or (a == b and 4*a == c):
        print("yes")
    else:
        print("no")
