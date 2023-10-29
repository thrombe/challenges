# Amicable numbers

from prob12 import primeFactors

def factors(n): # returns set(unsorted) of factors including 1 and itself
    allFactors = set()
    if n in {1, 0}: return set()
    for i in range(2, round(n**0.5) + 1):
        if not n % i: allFactors.update({i, n // i})
    return allFactors.union({1, n})

# **** VERY INEFFICIENT WAY TO GET FACTORS. INSTEAD JUST USE THE if not i %: list.add(i, num///i)
def factors2(n, primes): # returns set(unsorted) of factors including 1 and itself
    allFactors = set()
    allFactors.add(1)
    Pfactors = primeFactors(n, primes)
    for prime, power in Pfactors.items():
        for po in range(power+1):
            loop = allFactors.copy()
            for facs in loop:
                num = facs*(prime**po)
                if num % prime**(power+1) != 0: allFactors.add(num)
    return allFactors

def ans(n):
    # primes = [2]
    facList = {}
    amicsum = 0
    for i in range(1, n+1):
        facs = sum(factors(i), -i) # factors include the number itself, but amicable nums dont need it
        # facs = sum(factors(i, primes), -i) # factors include the number itself, but amicable nums dont need it
        facList.setdefault(facs, [])
        facList[facs].append(i)
    for amic, nums in facList.items():
        loop = nums.copy()
        for num in loop:
            nums.remove(num)
            if amic in facList.get(num, []):
                facList[num].remove(amic)
                amicsum += num + amic
                # print(num, factors(num, primes), amic)
    return amicsum

if __name__ == '__main__':
    from time import time
    start = time()
    print(ans(10000), time() - start)