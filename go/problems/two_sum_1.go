package main

func twoSum(nums []int, target int) []int {
	mapping := make(map[int]int)

	for i, val := range nums {
		compl := target - val
		if index, found := mapping[compl]; found {
			return []int{index, i}
		}
		mapping[val] = i
	}

	return nil
}
