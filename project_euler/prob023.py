# Non-abundant sums

from prob21 import factors


def ans(n):
    abunList = []
    for i in range(1, n):
        summ = sum(factors(i), -i)
        if summ > i: abunList.append(i)
    abunSumList = set()
    loop = len(abunList)-1 # dont try for last item
    for _ in range(loop):
        abunSumList.add(abunList[0]*2) # 12, 12 is considered a pair
        first = abunList.pop(0)
        if first + abunList[0] > n: break
        for i in abunList:
            abunSumList.add(i + first) # sum of pairs
    summ = 0
    for i in range(1, abunList[-1]):
        if i not in abunSumList:
            summ += i
    return summ

if __name__ == '__main__':
    from time import time
    start = time()
    print(ans(28125), time() - start) # 28123
