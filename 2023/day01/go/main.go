package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"unicode"
)

func readLines() []string {
	scanner := bufio.NewScanner(os.Stdin)
	var input []string
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	return input
}

func main() {
	input := readLines()
	PartOne(input)
	PartTwo(input)
}

/*
	Part One
*/

func PartOne(lines []string) {
	var total uint64

	for _, line := range lines {
		line_total, err := parseLineInts(line)
		if err != nil {
			fmt.Println("Failed to parse integer", err)
		}
		total += line_total
	}
	fmt.Println("Part One:", total)
}

func parseLineInts(line string) (uint64, error) {
	var nums []string
	for _, ch := range line {
		if unicode.IsNumber(ch) {
			nums = append(nums, string(ch))
		}
	}
	return strconv.ParseUint(nums[0]+nums[len(nums)-1], 10, 64)
}

func PartTwo(lines []string) {
	// Yuck, but it needs to be done. If I have to. I guess.
	values := map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}
	// Rather than come up with a perfect regex for the whoe thing, we'll
	// iterate through each `slice[idx:]` to see if it starts with one of the 
	// matches. This is important for cases where the numbers overlap, like `threeight`
	exp, _ := regexp.Compile(`^(\d|one|two|three|four|five|six|seven|eight|nine)`)
	
	var total uint64
	
	for _, line := range lines {
		var nums []string
		for idx := range line {
			match := exp.FindString(line[idx:])
			if match == "" {
				continue // skip iteration if the slice doesn't start with a match
			}
			nums = append(nums, match)
		}

		first := normalizeNums(values, nums[0])
		last := normalizeNums(values, nums[len(nums)-1])
		// We can reuse this guy from Part 1
		line_total, err := parseLineInts(first + last)
		if err != nil {
			fmt.Println("Failed to parse integer", err)
		}
		total += line_total
	}
	fmt.Println("Part Two:", total)
}

func normalizeNums(m map[string]string, k string) string {
	value := m[k]
	if value == "" {
		return k
	} else {
		return value
	}
}
