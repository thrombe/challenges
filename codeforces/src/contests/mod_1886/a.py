for _ in range(int(input())):
    n = int(input())

    if n < 7:
        print("NO")
    elif n%3 == 1 or n%3 == 2:
        print("YES")
        print(1, 2, n-3)
    elif n%3 == 0:
        if n == 9:
            print("NO")
        else:
            print("YES")
            print(1, 4, n-5)
    
