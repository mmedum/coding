package main

import (
	"strings"
)

func wordPattern(pattern string, s string) bool {
	patternToS := make(map[string]string)
	sToPattern := make(map[string]string)
	sSplit := strings.Split(s, " ")
	indexPattern := 0

	if len(pattern) != len(sSplit) {
		return false
	}

	for _, key := range sSplit {
		pattern := string(pattern[indexPattern])

		if val, ok := sToPattern[key]; ok && val != pattern {
			return false
		}

		if val, ok := patternToS[pattern]; ok && val != key {
			return false
		}

		patternToS[pattern] = key
		sToPattern[key] = pattern
		indexPattern++
	}

	return true
}
