https://leetcode.com/problems/gas-station/description/

## Naive

- Try each start point
    - go through the circle and check gas vs cost
    - TIME: O(n^2)
    - SPACE: O(1)

## Greedy

- P1: Check if there is a solution 
    - SUM(gas) >= SUM(cost)
- P2: find possible start
    - cost[i] >= gas[i]
