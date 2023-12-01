package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"unicode"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var input []string

	for scanner.Scan() {
		input = append(input, scanner.Text())
	}

	PartOne(input)
	PartTwo(input)
}

func PartOne(lines []string){
	var total uint
	for _,line := range lines {
		var nums []rune
		for _,ch := range line {
			if unicode.IsNumber(ch) {
				nums = append(nums, ch)
			}
		}
		cal_value := string([]rune{nums[0], nums[len(nums)-1]})
		line_total, err := strconv.ParseUint(cal_value, 10, 64)
		
		if err != nil {
			fmt.Println(cal_value, "can't be parsed as an integer.")
		}
		
		total += uint(line_total)
	}
	fmt.Println("Part One:", total)
}

func PartTwo(lines []string){
	fmt.Println("Part Two:", len(lines))
}