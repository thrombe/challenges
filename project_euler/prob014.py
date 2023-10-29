#Longest Collatz sequence

def collatz(i, known):
    n = i
    length = 0
    while True:
        if n == 1: break
        elif n in known.keys():
            length += known[n]
            break
        length += 1
        if n%2 == 0:
            n = n//2
        elif n%2 == 1:
            n = 3*n+1
    known[i] = length
    return length

def ans(n):
    known = {}
    longest = 1
    for i in range(2, n):
        length = collatz(i, known)
        if length > longest: longest, maxx = length, i
    # print(known)##
    return(maxx, longest)
import time
start = time.time()
print(ans(10**6), time.time()-start)