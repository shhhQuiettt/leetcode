package leetcode

func twoSum(nums []int, target int) []int {
	lookingFor := map[int]int{}
	for i, num := range nums {

		if firstIndex, ok := lookingFor[num]; ok {
			return []int{firstIndex, i}
		} else {
			lookingFor[target-num] = i
		}
	}
	return nil
}
