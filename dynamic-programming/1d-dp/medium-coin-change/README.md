- https://leetcode.com/problems/coin-change/
- Idea: With any value from 1 -> amount
    - Find the minimum coins sum up to that amount
- Create a dp to store the minimum number of coins that sum up to the value == idx
    - dp[i]
        - = -1 (not possible to sum up to this value)
        - = 1 (if i == coin value)
        - = min(dp[i - coin] && i - coin >= 0)