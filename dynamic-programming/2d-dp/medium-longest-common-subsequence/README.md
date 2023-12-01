> https://leetcode.com/problems/longest-common-subsequence/description/

## Problem

- Find the length of the longest common subsequence (LCS) between 2 strings

## Naive

- Generate all subsequences then compare it to find the longest common
    - Complexity:
        - Generate all subsequences: 2^n/string -> 2^n + 2^m
        - Find check all common
            - Put 1 string's subsquences into a set -> O(2^n) space
            - Check time: 2^n
        - Total time complexity: O(2^n + 2^m)
- [Solution](./go-sol/naive.go)

## 2D dynamic programming

- Given 2 string **ace** & **abcde**
    - Consider the first character is matched 
    - The sub-problem is the LCS between **ce** & **bcde**
- Given 2 string **ace** & **bbcde**
    - These first characters isn't matched
    - The sub-problems (remove the first charactor of the first string or the second string)
        - LCS between: **ce** & **bbcde**
        - LCS between: **ace** & **bcde**

### Visualize: 2D map

|       | a   | c   | e   | BOUND |
| ----- | --- | --- | --- | ----- |
| a     | ⇘   | -   | -   | 0     |
| b     | -   | ⇓   | -   | 0     |
| c     | -   | ⇘   | -   | 0     |
| d     | -   | -   | ⇓   | 0     |
| e     | -   | -   | 1   | 0     |
| BOUND | 0   | 0   | 0   | 0     |

- Directions: → = i; ↓ = j
    - map[i][j] = LCS(text1[i..], text2[j..])
    - The size of the considering map is the side of the sub-problem
    - => solve bottom up, from the smallest problem
- Formula
    - DP[i][j]
        - = DP[i-1][j+1] + 1 (if text1[i] == text1[j] <=> the character is matched, reduce the size of both strings)
        - = max(DP[i][j+1], DP[i+1][j]) (if text[i] != text[j] <=> the character isn't matched, reduce the size of 1 string)
- Complexity
    - Time: O(m*n) => need to calculate the full map
    - Space: O(m*n) => a map size m * n