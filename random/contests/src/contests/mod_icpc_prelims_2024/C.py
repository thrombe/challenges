def solve():
    t = int(input())  # number of test cases
    for _ in range(t):
        n = int(input())  # number of points
        x = list(map(int, input().split()))  # positions of the points
        a = list(map(int, input().split()))  # cost per unit length at each point
        
        # We need to pair each point with its cost and position, then sort by positions
        points = sorted(zip(x, a))
        
        # Initialize the total cost
        total_cost = 0
        
        # Traverse through sorted points and connect consecutive points
        for i in range(1, n):
            # Calculate the cost to connect points[i-1] and points[i]
            xi, ai = points[i-1]
            xj, aj = points[i]
            total_cost += min(ai, aj) * (xj - xi)
        
        # Output the result for the current test case
        print(total_cost)

solve()
