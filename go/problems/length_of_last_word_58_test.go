package main

import (
	"testing"
)

func TestLengthOfLastWord(t *testing.T) {
	val := "Hello World"
	want := 5
	result := lengthOfLastWord(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "   fly me   to   the moon  "
	want = 4
	result = lengthOfLastWord(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "luffy is still joyboy"
	want = 6
	result = lengthOfLastWord(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}

	val = "day"
	want = 3
	result = lengthOfLastWord(val)

	if result != want {
		t.Errorf("Expected %d, got %d", want, result)
	}
}
