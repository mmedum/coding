package main

func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}

	wordMap := make(map[rune]int)

	for _, key := range s {
		if val, found := wordMap[key]; found {
			wordMap[key] = val + 1
		} else {
			wordMap[key] = 1
		}
	}

	for _, tKey := range t {
		if val, found := wordMap[tKey]; found && val > 0 {
			wordMap[tKey] = val - 1
		} else {
			return false
		}
	}

	return true
}
