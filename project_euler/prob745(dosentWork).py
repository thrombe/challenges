# TOOOOO SOLW
'''
so i think i have a algo
    we generate prime numbers as needed according to N
    divide n by primes and we get a smaller no. 
    recursively divide it by that prime no till it dosent divide completely
    generate new prime and do the same
    now we got factors in a list
    we just take all factors out such that it makes a square
'''

import time
start = time.time()

def primeGen(primes, n):
    lastPrime = primes[-1]
    for i in range(1, int(n**0.5+1)):
        potentialPrime = lastPrime + i
        if any(potentialPrime % j == 0 for j in primes): continue
        primes.append(potentialPrime)
        return potentialPrime
    return n

def primeFactors(n):
    factors = {}
    for i in primes:
        factors.setdefault(i, 0)
        while n % i == 0:
            factors[i] += 1
            n = n//i
            if n == 1: break
        if n == 1: break
        if factors[i] == 0: factors.pop(i)
    while n != 1:
        newPrime = primeGen(primes, n)
        factors.setdefault(newPrime, 0)
        while n % newPrime == 0:
            factors[newPrime] += 1
            n = n//newPrime
        if factors[newPrime] == 0: factors.pop(newPrime)
    return factors

def sqr1(a):
    lim = int(a**(1/2))
    for i in range(lim):
        ba = (lim-i)**2
        if a % ba == 0: return ba
    return 1

def sqr2(n):
    factors = primeFactors(n)
    square = 1
    for prime, pow in factors.items():
        if pow % 2 == 0:
            square = square*(prime**pow)
        if pow % 2 == 1:
            square = square*(prime**(pow-1))
    return square

def s1(n):
    return sum(sqr1(i) for i in range(1, n+1))

def s2(n):
    return sum(sqr2(i) for i in range(1, n+1)) #somehow this is slower
primes = [2, 3]

a = 10**4 # for 10**5 -> 22910120
ver = 2
if ver == 1: print(s1(a))
else: print(s2(a))

print(time.time() - start)
