package main

import (
	"testing"
)

func TestRemoveElement(t *testing.T) {
	nums := []int{3, 2, 2, 3}
	val := 3
	want := 2
	result := removeElement(nums, val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	nums = []int{0, 1, 2, 2, 3, 0, 4, 2}
	val = 2
	want = 5
	result = removeElement(nums, val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	nums = []int{3, 2, 2, 3}
	val = 3
	want = 2
	result = removeElement(nums, val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

}
