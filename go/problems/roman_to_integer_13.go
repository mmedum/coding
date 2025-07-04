package main

func romanToInt(s string) int {
	romanTranslation := map[byte]int{
		'I': 1,
		'V': 5,
		'X': 10,
		'L': 50,
		'C': 100,
		'D': 500,
		'M': 1000,
	}

	sum := 0
	prev := 0
	for i := len(s) - 1; i >= 0; i-- {
		value := romanTranslation[s[i]]
		if value < prev {
			sum -= value
		} else {
			sum += value
		}
		prev = value
	}

	return sum
}
