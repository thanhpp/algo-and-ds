## Union Find algorithm

- Union(x, y): merge 2 groups containing x and y
- Find(x): returns the groups containing x

- Use in graph
    - Chose a representative for each group: result for the **find** function
        - usually represent graph as a tree
            - the representative = highest parent
    - Union: merge 2 tree -> 1 tree
        - 1 became the child of _**the other**_
        - all of the nodes in the children tree has find(x) = the representative of _**the other**_

- Time complexity: O(log(n)) - traverse the tree
- Space complexity: O(n)

## Problem

- https://leetcode.com/problems/redundant-connection/

- Solution: if an edge contains 2 nodes that have the same parent -> redundant