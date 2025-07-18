package main

import "testing"

func TestValidAnagram(t *testing.T) {
	input := "anagram"
	target := "nagaram"
	want := true
	result := isAnagram(input, target)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

	input = "rat"
	target = "car"
	want = false
	result = isAnagram(input, target)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

}
