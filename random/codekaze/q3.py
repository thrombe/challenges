from typing import *


def is_possible(chapter: int, progress: List[int], conditions: List[List[int]]) -> bool:
    p, *c = conditions[chapter]
    
    if progress[chapter]:
        return True
    
    if p > 0 and progress.count(True)*100 < len(progress) * p:
        return False
    
    for requirement in c:
        if requirement != -1 and not progress[requirement]:
            return False
    
    return True


def dfs(chapter: int, progress: List[int], conditions: List[List[int]]) -> bool:
    if chapter == len(conditions):
        return True
    
    if is_possible(chapter, progress, conditions):
        progress[chapter] = True
        
        if dfs(chapter + 1, progress, conditions):
            return True
        
        progress[chapter] = False
    
    return dfs(chapter + 1, progress, conditions)


def remembrance(m: int, n: int, conditions: List[List[int]]) -> int:
    progress = [False] * n
    
    if dfs(0, progress, conditions):
        # print(progress)
        # if all(progress):
            # return 1
        # else:
            # return 0
        return 1

    
    return 0


M = 2
N = 4
# conditions = [[20, -1, -1], [30, -1, -1]]
conditions = [[75, -1, 2], [10, 3, -1], [0, -1, -1], [25, -1, -1]]

result = remembrance(M, N, conditions)
print(result)

"""
m protagonists
n chapters

chapter conditions as p and c
p is percentage completion of story
c[m]  
"""


'''
you have just developed a video game that tells a non linear story from the perspective of m protagonists there are n different chapters in the story each chapter is seen from the perspective of one of the protagonists and has some conditions to be unlocked    

 the conditions for a chapter are given by the combination of an integer p and an array c

'P' represents the percentage of the overall story you must complete.
'C'  is an array of length 'M' such that 'C[i]' is a chapter from the 'i-th' protagonist's perspective that you must complete, or '-1' if there is no requirement.

the game launches tomorrow you want to perform one last check that it is possible to finish the game i e do all n chapters in an order that satisfies the conditions return 1 if such an order exists and 0 otherwise

Let 'M' = 2, 'N' = 3, conditions = [ [ 60, -1, -1 ], [ 0, -1, -1 ], [ 0, 1, -1 ] ].
The first chapter is not dependent on any specific chapter, but it needs 'P = 60%' total completion; Since there are '3' chapters, doing at least '2' will pass this requirement.
The second chapter has no requirements.
The third chapter does not have a total completion requirement, but it needs the second chapter completed.
Therefore, we can complete all 'N' chapters in the order: Second → Third → First.
Thus, the answer is '1'.

'''