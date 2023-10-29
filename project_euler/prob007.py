# 10001 th prime
def primeGen(n, primes = []): # generators are slow for this type of question. prefer a normal function that gives the nth prime instaed
    if not primes:
        primes.append(2)
        yield 2
    if len(primes) == 1:
        primes.append(3)
        yield 3
    lastPrime = primes[-1]
    i, j = 0, 2
    while j != n:
        i += 2
        potentialPrime = lastPrime + i
        limit = int(potentialPrime**0.5) +1
        if any(potentialPrime % j == 0 for j in primes if j < limit): continue
        # if any(j > limit or potentialPrime % j == 0 for j in primes): continue # donno why brokey
        primes.append(potentialPrime)
        j += 1
        yield potentialPrime

import time
start = time.time()

limit = 10001
for i in primeGen(limit):
    prime = i
print(prime, time.time() - start)