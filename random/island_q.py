#q- remove any islands of '1'
  # an island is a connected body of '1's not connected to the borders of the 2d array
  # only consider connections in up, down, left, right. no diagonal
  # example q input as 'matrix', and example q ans as 'output'

import copy
import pprint

def ans_islands(matrix):
    to_remove = copy.deepcopy(matrix)
    for i, ele in enumerate(to_remove[0]):
        if ele == 1:
            remove_if_connected(to_remove, i, 0)
    for i, ele in enumerate(to_remove[len(to_remove)-1]):
        if ele == 1:
            remove_if_connected(to_remove, i, len(to_remove)-1)
    for i in range(len(matrix)):
        ele = matrix[i][0]
        if ele == 1:
            remove_if_connected(to_remove, 0, i)
        ele = matrix[i][-1]
        if ele == 1:
            remove_if_connected(to_remove, len(to_remove[0])-1, i)
        
    for r in range(len(to_remove)):
        for c in range(len(to_remove[0])):
            if to_remove[r][c] == 1:
                matrix[r][c] = 0
    return matrix

def remove_if_connected(matrix, x, y):
    if x >= len(matrix[0]) or y >= len(matrix): return
    if x < 0 or y < 0: return
    if matrix[y][x] == 1:
        matrix[y][x] = 0
        remove_if_connected(matrix, x+1, y)
        remove_if_connected(matrix, x, y+1)
        remove_if_connected(matrix, x-1, y)
        remove_if_connected(matrix, x, y-1)

def test_islands():    
    matrix = [
        [1, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 1, 1],
        [0, 0, 1, 0, 1, 0],
        [1, 1, 0, 0, 1, 0],
        [1, 0, 1, 1, 0, 0],
        [1, 0, 0, 0, 0, 1],
        ]
    pprint.pprint(matrix)
    print("")
    output = [
        [1, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 1, 1],
        [0, 0, 0, 0, 1, 0],
        [1, 1, 0, 0, 1, 0],
        [1, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 1],
        ]
    
    pprint.pprint(ans_islands(matrix))
    print("")
    pprint.pprint(output)

if __name__ == "__main__":
    test_islands()
    pass
