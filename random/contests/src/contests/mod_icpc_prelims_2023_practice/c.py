n = int(input())
a = list(map(int, input().split()))
a.sort()

def count_beautiful_pairs(arr):
    n = len(arr)
    max_val = max(arr)
    freq = [0] * (max_val + 1)

    for num in arr:
        freq[num] += 1

    beautiful_pairs = 0

    for i in range(1, max_val + 1):
        for j in range(i * 2, max_val + 1, i):
            beautiful_pairs += freq[i] * freq[j]

    return beautiful_pairs    

print(count_beautiful_pairs(a))

# divides = [[] for _ in range(n)]
# for i in range(n-1, -1, -1):
#     e = a[i]
#     for j in range(i-1, -1, -1):
#         if a[i]%a[j] == 0:
#             a[j].append(i)
            
        
