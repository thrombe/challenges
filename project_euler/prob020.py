# Factorial digit sum

def factorial(n, tot = 1):
    for i in range(1, n+1):
        tot = tot*i
    return tot

def ans(n):
    return sum(list(int(i) for i in str(factorial(n))))

print(ans(100))