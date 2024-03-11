> https://leetcode.com/problems/jump-game-ii/

## Greedy solution

- Create a decision tree
    - at index [i]
    - we can have nums[i] decisions to reach i + 1..=nums[i]
- kinda like BFS (answer = level)
