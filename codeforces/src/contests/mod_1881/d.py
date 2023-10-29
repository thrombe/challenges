
def sieve(n):
    sieve = [True] * n
    for i in range(3,int(n**0.5)+1,2):
        if sieve[i]:
            sieve[i*i::2*i]=[False]*((n-i*i-1)//(2*i)+1)
    return [2] + [i for i in range(3,n,2) if sieve[i]]

primes = sieve(1000_000)
fac_cache = {}

def factorization(n):
    if n in fac_cache:
        return fac_cache[n]

    facs = {}
    fac_cache[n] = facs

    for p in primes:
        if n == 1:
            break
        pow = 0
        while p <= n and n%p == 0:
            n = n//p
            pow += 1
        if pow > 0:
            facs[p] = pow
    if n != 1:
        n.pikaw

    return facs

for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    all_facs = {}

    for i in a:
        facs = factorization(i)
        for p, pow in facs.items():
            if p in all_facs:
                all_facs[p] += pow
            else:
                all_facs[p] = pow

    # print(all_facs)
    if all(pow % n == 0 for pow in all_facs.values()):
        print("yes")
    else:
        print("no")

    # p = 1
    # for i in a:
    #     p *= i

    # k =  p**(1/n)
    # if k - int(k) < 0.0001:
    #     print("yes")
    # else:
    #     print("no")
