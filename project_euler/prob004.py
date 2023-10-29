# largest palindrome product

import time

def palinCheck(n):
    n = list(str(n))
    return all(n.pop(0) == n.pop(-1) for _ in range(len(n)//2))

def palinGen(n, palinMax = 0):
    for i in range(10**n-1, 10**(n-1)+1, -1):
        for j in range(10**n-1, 10**(n-1)+1, -1):
            num = (i)*(j)
            if palinCheck(num):
                if palinMax < num: palinMax, fac1, fac2 = num, i, j
    return palinMax, fac1, fac2

def palinGen2(palinMax = 0): # palindrome == abccba for 6 char no. 100001a + 10010b + 1100c = 11(9091a + 910b + 100c) # this is specific for 6 char things only
    for i in range(999, 100, -1):
        for j in range(990, 110, -11):
            num = (i)*(j)
            if palinCheck(num):
                if palinMax < num: palinMax, fac1, fac2 = num, i, j
    return palinMax, fac1, fac2

def palinGen3(m, palinMax = 0): # tests from the top 10 numbers then 10**2 numbers then 10**3 .......
    for n in range(m-1, 0, -1):
        for i in range(10**m-1, 10**m-10**(m-n), -1):
            for j in range(10**m-1, 10**m-10**(m-n), -1):
                num = (i)*(j)
                if palinCheck(num):
                    if palinMax < num: palinMax, fac1, fac2 = num, i, j
        if palinMax != 0: return palinMax, fac1, fac2

''' #broken
def palinGen4(m, palinMax = 0): # tests for numbers of index -1 to -10 then -10 to -20 then -20 to -30 .... then -1 to -1*10**2 then -1*10**2 to -2*10**2 ......
    for n in range(m-1, 0, -1):
        for k in range(1, 11):
            for i in range(10**m-10**(m-n)*(k-1), (10**m-10**(m-n)*k), -1):
                # print(i, n)##
                for j in range(10**m-10**(m-n)*(k-1), (10**m-10**(m-n)*k), -1):
                    num = (i)*(j)
                    if palinCheck(num):
                        if palinMax < num: palinMax, fac1, fac2 = num, i, j
            if palinMax: return palinMax, fac1, fac2

start = time.time() # mine.
print(palinGen4(3), time.time() - start)
'''

start = time.time() # mine. crushes till n = 6 in 5 seconds and n=7 in 500 seconds
print(palinGen3(3), time.time() - start)

start = time.time() # not mine. limited to n = 3
print(palinGen2(), time.time() - start)

start = time.time() # mine. bad
print(palinGen(3), time.time() - start)