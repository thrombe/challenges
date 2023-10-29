# largest prime factor

def primeGen(primes, n):
    if not primes:
        primes.append(2)
        return 2
    elif len(primes) == 1:
        primes.append(3)
        return 3
    lastPrime = primes[-1]
    for i in range(2, int(n**0.5+1), 2):
        potentialPrime = lastPrime + i
        if any(potentialPrime % j == 0 for j in primes): continue
        primes.append(potentialPrime)
        return potentialPrime
    return n

def primeFactors(n):
    primes = []
    while n != 1:
        newPrime = primeGen(primes, n)
        while n % newPrime == 0:
            n = n//newPrime
    return newPrime

import time
start = time.time()

print(primeFactors(600851475143))

print(time.time() - start)