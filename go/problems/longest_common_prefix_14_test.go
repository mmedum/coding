package main

import (
	"testing"
)

func TestLongestCommonPrefix(t *testing.T) {
	vals := []string{"flower", "flow", "flight"}
	want := "fl"
	result := longestCommonPrefix(vals)

	if result != want {
		t.Errorf("Expected %s, got %s", want, result)
	}

	vals = []string{"dog", "racecar", "car"}
	want = ""
	result = longestCommonPrefix(vals)

	if result != want {
		t.Errorf("Expected %s, got %s", want, result)
	}

	vals = []string{}
	want = ""
	result = longestCommonPrefix(vals)

	if result != want {
		t.Errorf("Expected %s, got %s", want, result)
	}
}
