
# from typing import *

# """"
# n vertices
# m edges
# k colors
# c[i] is ith color

# """

# def colorful_graph(n: int, m: int, edges: List[List[int]], k: int, c: List[int]) -> int:
#     # write your code here
#     pass


from typing import List

def colorful_graph(n: int, m: int, edges: List[List[int]], k: int, c: List[int]) -> int:
    operations = 0

    itocomp = {}
    isin = set()
    comps = []

    for u, v in edges:
        cu = c[u]
        cv = c[v]
        
        if cu != cv:
            operations += 1
            isin.add(u)
            isin.add(v)
            comps.append({u})
            itocomp[u] = len(comps) -1
            comps.append({v})
            itocomp[v] = len(comps) -1
        else:
            if u not in isin and v not in isin:
                isin.add(u)
                isin.add(v)
                comps.append({u, v})
                itocomp[u] = len(comps) -1 
                itocomp[v] = len(comps) -1
            elif u in isin and v not in isin:
                isin.add(v)
                itocomp[v] = itocomp[u]
                comps[itocomp[u]].add(v)
            elif v in isin and u not in isin:
                isin.add(u)
                itocomp[u] = itocomp[v]
                comps[itocomp[v]].add(u)
            else:
                if itocomp[u] == itocomp[v]:
                    continue
                comps[itocomp[u]] |= comps[itocomp[v]]
                comps[itocomp[v]] = None
                itocomp[v] = itocomp[u]

    for i, color in enumerate(c):
        if i not in isin:
            found = False
            for comp in comps:
                if comp is not None:
                    r = comp.pop()
                    comp.add(r)
                    comp_c = c[r]
                    if comp_c == color:
                        found = True
                        comp.add(i)
                        operations += 1
                        break
            if not found:
                isin.add(i)
                comps.append({i})
                itocomp[i] = len(comps) - 1

    components = {i: set() for i in range(k)}
    for i, comp in enumerate(comps):
        if comp is None:
            continue
        r = comp.pop()
        comp.add(r)
        comp_color = c[r]
        components[comp_color].add(i)

    print(components)

    for s in components.values():
        if len(s) > 0:
            operations += len(s) - 1

    return operations


print(colorful_graph(7, 7, [[0, 1], [1, 2], [1, 3], [2, 4], [2, 5], [2, 6], [3, 5]], 5, [0, 1, 2, 1, 3, 3, 3]))

"""
you are given a graph consisting of n vertices and m edges there are k colors you are also given an array c of length n such that c i is the color of the i th vertex    

in one operation you can add or remove an edge between any two vertices

find the minimum number of operations you need such that any two vertices with the same color belong to the same connected component and any two vertices with different colors belong to different connected components

for example:
Let 'N' = 3, 'M' = 2, edges = [ [ 0, 1 ], [ 1, 2 ] ], 'K' = 2, 'C' = [ 0, 1, 0 ].
Both the input edges connect vertices of different colors. Thus, we remove them.
Then to connect both vertices having color '0', ie. vertices '0' and '2',  we add the edge [ 0, 2 ].
Therefore, the minimum number of operations is '3'.


"""