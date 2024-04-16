package gosol

func minOperations(nums []int) int {
    var opCount = 0
    for i := 1; i < len(nums); i++ {
        if nums[i] > nums[i - 1] {
            continue
        }

        c := nums[i - 1] - nums[i] + 1
        opCount += c
        nums[i] += c
    }

    return opCount
}
