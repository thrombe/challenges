# Highly divisible triangular number

def triangleNum(n):
    return (n+1)*n//2

def primeGen(primes, n): # inefficient prime generator, plz improve
    lastPrime = primes[-1]
    for i in range(1, int(n**0.5+1)):
        potentialPrime = lastPrime + i
        if any(potentialPrime % j == 0 for j in primes): continue
        primes.append(potentialPrime)
        return potentialPrime
    return n

def primeFactors(n, primes):
    factors = {}
    for i in primes:
        factors.setdefault(i, 0)
        while n % i == 0: # if it is dividible, then divide else move to next prime
            factors[i] += 1
            n = n//i
            if n == 1: break
        if n == 1: break
        if factors[i] == 0: factors.pop(i)
    while n != 1: # if primes are not already in primes, generate new primes and check for req
        newPrime = primeGen(primes, n)
        factors.setdefault(newPrime, 0)
        while n % newPrime == 0:
            factors[newPrime] += 1
            n = n//newPrime
        if factors[newPrime] == 0: factors.pop(newPrime)
    return factors

def ans(n):
    i = 1
    primes = [2]
    while True:
        i += 1
        factors = primeFactors(triangleNum(i), primes)
        tot = 1
        for j in factors.values():
            tot = tot*(j+1)
        if tot >= 500: break
    return i, triangleNum(i), tot

if __name__ == '__main__':
    import time
    start = time.time()
    print(ans(7), time.time()-start)