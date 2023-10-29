# summation of primes

def sieve1(n): # 3.69 sec
    potentialPrimes = [False]+[True]*(n-1)
    primes = []
    for i, prime in enumerate(potentialPrimes):
        if prime == False: continue
        elif prime == True:
            primes.append(i+1)
            num = 2*(i+1)
            while num <= n:
                potentialPrimes[num-1] = False
                num += i+1
    return primes

def sieve2(n): # 1.26 sec
    potentialPrimes = [False]*2+[True]*(n-1)
    primes = []
    limit = int(n**.5)+1
    for i, prime in zip(range(limit), potentialPrimes):
        if prime == True:
            primes.append(i)
            for num in range(i**2, n+1, i):
                potentialPrimes[num] = False
    for i, isprime in zip(range(limit, n+1), potentialPrimes[limit : ]):
        if isprime: primes.append(i)
    return primes

def sieve3(n): # 0.86 sec
    n = n+1
    potentialPrimes = [False]+[True]*(n//2-1) # for 3, 5, 7, 9, .....
    primes = [2]
    limit = int(n**.5)+1
    for i in range(3, limit, 2):
        prime = potentialPrimes[i//2]
        if prime is True:
            primes.append(i)
            for num in range(i**2, n, i*2):
                potentialPrimes[num//2] = False
    if limit %2 == 0: limit -= 1
    for i in range(limit, n, 2):
        if potentialPrimes[i//2]: primes.append(i)
    return primes

def sieve4(n): # 1.233 sec
    n = n+1
    potentialPrimes = [False]+[True]*(n//2-1) # for 3, 5, 7, 9, .....
    primes = [2]
    limit = int(n**.5)+1
    for i, prime in zip(range(3, limit, 2), potentialPrimes[1: limit//2]):
        if prime is True:
            primes.append(i)
            for num in range(i**2, n, i*2):
                potentialPrimes[num//2] = False
    if limit %2 == 0: limit -= 1
    for i, prime in zip(range(limit, n, 2), potentialPrimes[limit//2 : ]):
        if prime: primes.append(i)
    return primes

num = 2*(10**6)

import time
start = time.time()
print(sum(sieve3(num)), time.time() - start)

###########

def primeGen(n, primes = []):
    primes.extend([2, 3])
    for i in range(2, n, 2):
        potentialPrime = 3 + i
        if any(potentialPrime % j == 0 for j in primes): continue
        primes.append(potentialPrime)
    return primes

def sieveBad(n):
    primes = [2]
    numbers = list(range(3, n+1, 2))
    while numbers:
        prime = numbers.pop(0)
        primes.append(prime)
        numbers = [num for num in numbers if num % prime != 0]
    return primes

#################

def sieve_for_primes_to(n): # 0.34 sec
    size = n//2
    sieve = [1]*size
    limit = int(n**0.5)
    for i in range(1,limit):
        if sieve[i]:
            val = 2*i+1
            tmp = ((size-1) - i)//val 
            sieve[i+val::val] = [0]*tmp
    return [2] + [i*2+1 for i, v in enumerate(sieve) if v and i>0]
start = time.time()
print(sum(sieve_for_primes_to(num)), time.time()-start)

def primes(n): # 0.31 sec
    sieve = [True] * n
    for i in range(3,int(n**0.5)+1,2):
        if sieve[i]:
            sieve[i*i::2*i]=[False]*((n-i*i-1)//(2*i)+1)
    return [2] + [i for i in range(3,n,2) if sieve[i]]
start = time.time()
print(sum(primes(num)), time.time()-start)