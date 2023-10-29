for _ in range(int(input())):
    n, p = map(int, input().split())
    a = list(map(int, input().split()))
    b = list(map(int, input().split()))

    ps = [(a, b) for a, b in zip(a, b)]
    ps.sort(key=lambda x: x[1] + 1/x[0])
    # print(ps)

    # cost = 0
    # costs = []
    # while len(ps) > 0:
    #     # print(ps, cost, costs)
    #     if len(costs) > 0 and costs[0] <= p:
    #         cost += costs[0]
    #         costs = costs[1:]
    #     else:
    #         cost += p

    #     t = ps[0]
    #     ps = ps[1:]
    #     if len(costs) < len(ps) and t[1] < p:
    #         costs += [t[1]]*t[0]

    cost = 0
    using = 0
    left = ps
    while len(left) > 0:
        # print(left, cost, using)
        if len(ps) == len(left):
            cost += p
            left = left[1:]
        elif ps[using][1] > p:
            cost += p*len(left)
            left = []
        else:
            cost += ps[using][1]*min(ps[using][0], len(left))
            left = left[min(len(left), ps[using][0]):]
            using += 1
            

    print(cost)


    
