# Power digit sum

def ans(n): return sum(int(i) for i in str(n))
print(ans(2**1000))