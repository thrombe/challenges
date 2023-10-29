# lattice paths

def factorial(n):
    result = 1
    for i in range(1, n+1):
        result = result*i
    return result

def ans(n):
    return factorial(2*n)//(factorial(n)**2)

print(ans(20))