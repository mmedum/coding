package main

func strStr(haystack string, needle string) int {
	i, end := 0, len(haystack)

	for i+len(needle) <= end {
		if haystack[i:i+len(needle)] == needle {
			return i
		}
		i++

	}

	return -1
}
