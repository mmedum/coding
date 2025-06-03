package main

func lengthOfLastWord(s string) int {
	count := 0
	processedWord := false
	for i := len(s) - 1; i >= 0; i-- {
		if string(s[i]) != " " {
			count += 1
			processedWord = true
		} else if processedWord {
			break
		}
	}

	return count
}
