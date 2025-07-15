package main

import (
	"testing"
)

func TestWordPattern(t *testing.T) {
	valPattern := "abba"
	val := "dog cat cat dog"
	want := true
	result := wordPattern(valPattern, val)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

	valPattern = "abba"
	val = "dog cat cat fish"
	want = false
	result = wordPattern(valPattern, val)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

	valPattern = "aaaa"
	val = "dog cat cat dog"
	want = false
	result = wordPattern(valPattern, val)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

	valPattern = "abba"
	val = "dog dog dog dog"
	want = false
	result = wordPattern(valPattern, val)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

	valPattern = "aaa"
	val = "aa aa aa aa"
	want = false
	result = wordPattern(valPattern, val)

	if result != want {
		t.Errorf("Expected %t, got %t", want, result)
	}

}
