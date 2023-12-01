package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"unicode"
)

func readLines()([]string){
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

func PartOne(lines []string){
	var total uint64

	for _,line := range lines {
		line_total, err := parseLineInts(line)
		if err != nil {
			fmt.Println("Failed to parse integer", err)
		}
		total += line_total
	}
	fmt.Println("Part One:", total)
}

func parseLineInts(line string)(uint64, error){
	var nums []string
	for _,ch := range line {
		if unicode.IsNumber(ch) {
			nums = append(nums, string(ch))
		}
	}
	return strconv.ParseUint(nums[0] + nums[len(nums)-1], 10, 64) 
}

func PartTwo(lines []string){
	
	values := map[string]string {
		"one": "1",
		"two": "2",
		"three": "3",
		"four": "4",
		"five": "5",
		"six": "6",
		"seven": "7",
		"eight": "8",
		"nine": "9",
	}

	exp, _ := regexp.Compile(`^(\d|one|two|three|four|five|six|seven|eight|nine)`)
	var total uint
	for _, line := range lines {

		var nums []string
		for idx := range line {
			match := exp.FindString(line[idx:])
			if match != "" {
				nums = append(nums, match)
			}
		}
		first := values[nums[0]]
		if first == "" {
			first += nums[0] 
		}

		last := values[nums[len(nums)-1]]
		if last == "" {
			last += nums[len(nums)-1]
		}

		cal_value := first + last

		line_total, err := strconv.ParseUint(cal_value, 10, 64)
			
		if err != nil {
			fmt.Println(cal_value, "can't be parsed as an integer.")
		}
		total += uint(line_total)
	}
	fmt.Println("Part Two:", total)
}