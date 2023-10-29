# special pythagorian triplet

def ans(summ):
    for i in range(1, summ//2):
        for j in range(1, summ//2):
            k = summ-i-j
            if i**2 == j**2 + k**2: return i*j*k

import time
start = time.time()
print(ans(1000), time.time() - start)
