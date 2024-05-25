package gosol

import "fmt"

func waysToMakeFair(nums []int) int {
	var (
		count           = 0
		prefixSum       = make([]int, len(nums))
		oddSum, evenSum = 0, 0
	)

	for i := 0; i < len(nums); i++ {
		if i%2 == 0 {
			evenSum += nums[i]
		} else {
			oddSum += nums[i]
		}

		if i >= 2 {
			prefixSum[i] = prefixSum[i-2] + nums[i]
		} else {
			prefixSum[i] = nums[i]
		}
	}

	fmt.Println(prefixSum, evenSum, oddSum)

	for i := 0; i < len(nums); i++ {
		// without i
		if i%2 == 0 {
			var prevOddSum, prevEvenSum = 0, 0
			// remove an even-indexed
			prevEvenSum = prefixSum[i] - nums[i]
			if i > 1 {
				prevOddSum = prefixSum[i-1]
			}
			// after i, odd <-> even
			originalAfterEvenSum := evenSum - prevEvenSum - nums[i]
			originalAfterOddSum := oddSum - prevOddSum

			if prevEvenSum+originalAfterOddSum == prevOddSum+originalAfterEvenSum {
				count += 1
			}

			// fmt.Println("i", i, "prevEvenSum", prevEvenSum, "prevOddSum", prevOddSum, "originalAfterEvenSum", originalAfterEvenSum, "originalAfterOddSum", originalAfterOddSum)
		} else {
			var prevOddSum, prevEvenSum = 0, 0
			// remove an odd-indexed
			prevOddSum = prefixSum[i] - nums[i]
			if i > 0 {
				prevEvenSum = prefixSum[i-1]
			}
			// after i, odd <-> even
			originalAfterEvenSum := evenSum - prevEvenSum
			originalAfterOddSum := oddSum - prevOddSum - nums[i]

			if prevEvenSum+originalAfterOddSum == prevOddSum+originalAfterEvenSum {
				count += 1
			}
			// fmt.Println("i", i, "prevEvenSum", prevEvenSum, "prevOddSum", prevOddSum, "originalAfterOddSum", originalAfterOddSum, "originalAfterOddSum", originalAfterOddSum)
		}
	}

	return count
}

/*
func waysToMakeFair(nums []int) int {
    count := 0

    // brute force
    for i := 0; i < len(nums); i++ {
        // remove i
        var oddSum, evenSum = 0, 0
        for j := 0; j < len(nums); j++ {
            if j == i {
                continue
            }

            if j % 2 == 0 {
                if j < i {
                    evenSum += nums[j]
                } else {
                    oddSum += nums[j]
                }
            } else {
                if j < i {
                    oddSum += nums[j]
                } else {
                    evenSum += nums[j]
                }
            }
        }

        if oddSum == evenSum {
            count += 1
        }
    }

    return count
}
*/
