import math
def dist(x1, y1, x2, y2):
    x = x2-x1
    y = y2-y1
    return math.sqrt(x*x + y*y)


for _ in range(int(input())):
    px, py = map(int, input().split())
    ax, ay = map(int, input().split())
    bx, by = map(int, input().split())

    dpa = dist(px, py, ax, ay)
    dpb = dist(px, py, bx, by)
    doa = dist(0, 0, ax, ay)
    dob = dist(0, 0, bx, by)
    h = dist(ax, ay, bx, by)/2

    possible = [max(dpa, doa), max(dpb, dob), max([h, dpa, dob]), max([h, doa, dpb])]
    
    # if (doa < dob and dpa > dpb):
    #     p = 
    #     possible.append(p)
    # else:
    #     h = dist(ax, ay, bx, by)/2
    #     p = max([h, doa, dpb])
    #     possible.append(p)
    p = min(possible)
    print(p)
