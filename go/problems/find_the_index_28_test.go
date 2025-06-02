package main

import "testing"

func TestStrStr(t *testing.T) {
	haystack := "sadbutsad"
	needle := "sad"
	want := 0
	result := strStr(haystack, needle)
	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	haystack = "leetcode"
	needle = "leeto"
	want = -1
	result = strStr(haystack, needle)
	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	haystack = "hello"
	needle = "ll"
	want = 2
	result = strStr(haystack, needle)
	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	haystack = "a"
	needle = "a"
	want = 0
	result = strStr(haystack, needle)
	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

}
