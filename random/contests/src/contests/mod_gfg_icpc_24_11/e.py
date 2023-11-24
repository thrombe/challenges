x = int(input())
nums = []
for _ in range(x):
    a = list(map(int, input().split()))
    nums.extend(a)

nums.sort()
print(*nums)


