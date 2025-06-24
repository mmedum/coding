package main

import "sort"

func longestCommonPrefix(strs []string) string {
	if len(strs) == 0 {
		return ""
	}

	sort.Strings(strs)

	first := strs[0]
	last := strs[len(strs)-1]

	index := 0
	for index < len(first) && index < len(last) && first[index] == last[index] {
		index++
	}

	return first[:index]
}
