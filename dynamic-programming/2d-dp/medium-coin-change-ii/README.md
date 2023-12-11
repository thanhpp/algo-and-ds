## Example 

- amount = 5, coins = [1,2,5]

|     | 5     | 4   | 3   | 2   | 1   |
| --- | ----- | --- | --- | --- | --- |
| 5   | **4** | 3   | 2   | 2   | 1   |
| 2   | 3     | 3   | 2   | 2   | 1   |
| 1   | 1     | 1   | 1   | 1   | 1   |

- dp[token][amount] 
    - += dp[token-1][amount]: not using the current token
    - += dp[token][amount-token]: use this token one more time
    - += 1 (amount % token): use only this token