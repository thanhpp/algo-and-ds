> https://leetcode.com/problems/jump-game-ii/

## Greedy solution

- With each level, determine the range of the next level
- Create a decision tree
    - at index [i]
    - we can have nums[i] decisions to reach i + 1..=nums[i]
- kinda like BFS (answer = level)
