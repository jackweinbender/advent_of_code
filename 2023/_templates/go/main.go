package main

import (
	"fmt"
	"shared"
)

func PartOne(lines []string)(any){
	return len(lines)
}
func PartTwo(lines []string)(any){
	return len(lines)
}

func main() {
	input := shared.ReadLines()
	fmt.Println("Part One:", PartOne(input))
	fmt.Println("Part Two:", PartTwo(input))
}