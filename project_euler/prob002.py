# even fibonacci numbers
def ans(n):
    a, b, sum = 0, 1, 0
    while True:
        a, b = b, a + b
        if b > n: return sum
        elif b % 2 == 0: sum += b

def ans2(n): # x, y, x+y, x+2y, 2x+3y, 3x+5y, 5x+8y, 8x+13y, 13x+21y, ......
    x, y, sum = 1, 1, 0 # notice that every third term is even 1, 1, 2, 3, 5, 8, 13, 21, 34, ..... # x+y is the third term
    while y < n: sum, x, y = sum+x+y, x+2*y, 2*x+3*y # moving x and y 3 steps ahead of their current value (notice that x+2y is always 3 steps ahead of x)
    return sum

def ans3(n):
    phi3, y, sum = ((1+5**0.5)/2)**3, 2, 0
    while y < n: sum, y = sum+y, round(y*phi3)
    return sum


import time
start = time.time()
print(ans(4*10**6), time.time() - start)

start = time.time()
print(ans2(4*10**6), time.time() - start)

start = time.time()
print(ans3(4*10**6), time.time() - start)