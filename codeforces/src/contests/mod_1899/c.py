def SubarrayWithMaxSum(nums):
 
    # Initialize currMax and globalMax
    # with first value of nums
    currMax = nums[0]
    globalMax = nums[0]
    # Initialize endIndex startIndex,globalStartIndex
    endIndex = 0
    startIndex = 0
    globalMaxStartIndex = 0
 
    # Iterate for all the elements of the array
    for i in range(1, len(nums)):
 
        # Update currMax and startIndex
        if (nums[i] > nums[i] + currMax):
            currMax = nums[i]
            startIndex = i  # update the new startIndex
 
        # Update currMax
        elif (nums[i] < nums[i] + currMax):
            currMax += nums[i]
 
        # Update globalMax and globalMaxStartIndex
        if (currMax > globalMax):
            globalMax = currMax
            endIndex = i
            globalMaxStartIndex = startIndex
    return sum(nums[globalMaxStartIndex:endIndex+1])


# Function to find the maximum subarray sum for a subarray that crosses the midpoint
# def crossover(arr):
#     # Calculate the midpoint of the array
#     mid = len(arr)//2

#     # Running sum for the left subarray
#     lsum = 0
#     # Maximum sum for the left subarray
#     lmax = 0
#     # Traverse the left subarray in reverse
#     p = 3
#     for i in arr[:mid][::-1]:
#         if p == i%2:
#             break
#         p = i%2
#         lsum += i
#         lmax = max(lsum, i)

#     # Running sum for the right subarray
#     rsum = 0
#     # Maximum sum for the right subarray
#     rmax = 0
#     p = 3
#     for i in arr[mid:]:
#         if p == i%2:
#             break
#         p = i%2
#         rsum += i
#         rmax = max(rsum, i)

#     if mid-1 >=0 and arr[mid-1]%2 == arr[mid]%2:
#         return max(lmax, rmax)

#     return lmax + rmax

# # Function to find the maximum subarray sum for the entire array using divide and conquer
# def max_subarray(arr):
#     if len(arr) <= 1:
#         return sum(arr)
    
#     # Calculate the midpoint of the array
#     mid = len(arr)//2

#     # Recursively find the maximum subarray sums for the left and right subarrays
#     lsum = max_subarray(arr[:mid])
#     rsum = max_subarray(arr[mid:])

#     # Find the maximum subarray sum for a subarray that crosses the midpoint
#     csum = crossover(arr)

#     # Return the maximum of the left subarray, right subarray, and crossover sum
#     return max([lsum, rsum, csum])

def crossover(arr):
    # Calculate the midpoint of the array
    mid = len(arr)//2

    # Running sum for the left subarray
    lsum = 0
    # Maximum sum for the left subarray
    lmax = 0
    # Traverse the left subarray in reverse
    for i in arr[:mid][::-1]:
        lsum += i
        lmax = max(lsum, lmax)

    # Running sum for the right subarray
    rsum = 0
    # Maximum sum for the right subarray
    rmax = 0
    for i in arr[mid:]:
        rsum += i
        rmax = max(rsum, rmax)

    # print("lr", lmax, rmax, arr)
    return lmax + rmax

# Function to find the maximum subarray sum for the entire array using divide and conquer
def max_subarray(arr):
    if len(arr) <= 1:
        return sum(arr)
    
    # Calculate the midpoint of the array
    mid = len(arr)//2

    # Recursively find the maximum subarray sums for the left and right subarrays
    lsum = max_subarray(arr[:mid])
    rsum = max_subarray(arr[mid:])

    # Find the maximum subarray sum for a subarray that crosses the midpoint
    csum = crossover(arr)

    # print("left: ", lsum, arr[:mid])
    # print("right: ", rsum, arr[mid:])
    # print("crossover: ", csum)
    
    # Return the maximum of the left subarray, right subarray, and crossover sum
    return max([lsum, rsum, csum])


for _ in range(int(input())):
    n = int(input())
    a = list(map(int, input().split()))

    # m = SubarrayWithMaxSum(a)
    arrs = []
    start = 0
    p = 3
    for (i, e) in enumerate(a):
        if p == e%2:
            arrs.append(a[start:i])
            start = i
        p = e%2
    arrs.append(a[start:n])

    m = -float('inf')
    for e in arrs:
        # print(e)
        m = max(m, max_subarray(e))
    print(m)

