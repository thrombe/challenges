# multiples of 3 and 5
def ans(n):
    return sum(i for i in range(1, n) if i % 3 == 0 or i % 5 == 0)
print(ans(1000))