# smallest multiple

def primeGen(n, primes = []):
    if not primes:
        primes.append(2)
        yield 2
    if len(primes) == 1:
        primes.append(3)
        yield 3
    lastPrime = primes[-1]
    for i in range(2, n, 2):
        potentialPrime = lastPrime + i
        if any(potentialPrime % j == 0 for j in primes): continue
        primes.append(potentialPrime)
        yield potentialPrime

def ans1(limit):
    ans = 1
    for prime in primeGen(limit): # generating primes as needed.
        primeE = 1
        while True:
            primeE = primeE*prime
            if primeE > limit:
                primeE = primeE//prime
                break
        ans = ans*primeE
    return ans

import time

start = time.time()
print(ans1(20), time.time() - start)