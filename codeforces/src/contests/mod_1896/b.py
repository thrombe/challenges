for _ in range(int(input())):
    n = int(input())
    s = input()
    s = s.lstrip('B')
    s = s.rstrip('A')

    print(max(0, len(s) - 1))
    

    # ans = 0
    # k = 'A'
    # while len(s) > 0:
    #     c = 0
    #     for kk in s:
    #         if k == kk:
    #             c += 1
    #         else:
    #             break
    #     s = s[c:]
    #     print(s)



        
