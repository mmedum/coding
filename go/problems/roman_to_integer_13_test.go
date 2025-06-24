package main

import (
	"testing"
)

func TestRomanToInteger(t *testing.T) {
	val := "III"
	want := 3
	result := romanToInt(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "LVIII"
	want = 58
	result = romanToInt(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "IV"
	want = 4
	result = romanToInt(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "MCMXCIV"
	want = 1994
	result = romanToInt(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}
}
