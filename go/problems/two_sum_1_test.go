package main

import (
	"testing"
)

func TestTwoSum(t *testing.T) {
	nums := []int{2, 7, 11, 15}
	target := 9
	want := []int{0, 1}
	result := twoSum(nums, target)

	if !isEqualArray(result, want) {
		t.Errorf("Expected %v, got %v", want, result)
	}

	nums = []int{3, 2, 4}
	target = 6
	want = []int{1, 2}
	result = twoSum(nums, target)

	if !isEqualArray(result, want) {
		t.Errorf("Expected %v, got %v", want, result)
	}

	nums = []int{3, 3}
	target = 6
	want = []int{0, 1}
	result = twoSum(nums, target)

	if !isEqualArray(result, want) {
		t.Errorf("Expected %v, got %v", want, result)
	}
}

func isEqualArray(a []int, b []int) bool {
	if len(a) != len(b) {
		return false
	}

	for i := range len(a) {
		if a[i] != b[i] {
			return false
		}
	}

	return true
}
