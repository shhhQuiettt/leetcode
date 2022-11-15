package leetcode

func removeDuplicates(nums []int) int {
	p := 0
	q := 1

	for q < len(nums) {
		if nums[p] != nums[q] {
			p++
			nums[p] = nums[q]
		}
		q++
	}
	return p + 1
}
