package main

import (
	"fmt"
	"shared"
	"unicode"
)

type Coordinate struct {
	row int
	col int
}

type PartNumber struct {
	loc Coordinate
	length int
}

type Graph []string

func isSymbol(c Coordinate, g Graph)(bool){
	v := rune(g[c.row][c.col])
	if unicode.IsNumber(v) || v == '.' {
		return false
	}
	return true
}

func getNumberBoundary(n PartNumber, g Graph)([]Coordinate){
	rindex := n.loc.col - 1
	lindex := n.loc.col + n.length
	
	coords := []Coordinate{}
	// This is gross, there must be some math for this
	coords = append(coords, Coordinate{ row: n.loc.row, col: lindex })
	coords = append(coords, Coordinate{ row: n.loc.row+1, col: lindex })
	coords = append(coords, Coordinate{ row: n.loc.row-1, col: lindex })

	coords = append(coords, Coordinate{ row: n.loc.row, col: rindex })
	coords = append(coords, Coordinate{ row: n.loc.row+1, col: rindex })
	coords = append(coords, Coordinate{ row: n.loc.row-1, col: rindex })

	for i := 0; i < n.length; i++ {
    coords = append(coords, Coordinate{ row: n.loc.row-1, col: n.loc.col+i })
		coords = append(coords, Coordinate{ row: n.loc.row+1, col: n.loc.col+i })
	}
	return coords
}

func touchesSymbol(n PartNumber, g Graph)(bool){
	
	for _,coord := range getNumberBoundary(n, g) {
		if coord.row < 0 || coord.col < 0 {
			continue
		}
		if coord.row >= len(g) || coord.col >= len(g[0]){
			continue
		}
		if isSymbol(coord, g){
			return true
		}
	}
	return false
}

func valueOf(n PartNumber, graph Graph)(int){
	str_slice := graph[n.loc.row][n.loc.col:n.loc.col+n.length]
	number, _ := shared.ParseInt(str_slice)
	
	return int(number)
}

func getNumbers(g Graph)([]PartNumber){
	numbers := []PartNumber{}

	for row,line := range g {
		in_number := false
		for col,ch := range line {
			if in_number {
				if unicode.IsNumber(ch){
					numbers[len(numbers)-1].length += 1
				} else {
					in_number = false
				}
			} else {
				if unicode.IsNumber(ch){
					new_num := PartNumber{length: 1, loc: Coordinate{row: row, col: col }}
					numbers = append(numbers, new_num)
					in_number = true
				} else {
					// noop
				}
			}
		}
	}
	return numbers
}

func PartOne(lines []string){
	numbers := getNumbers(lines)
	total := 0
	for _,number := range numbers {
		if touchesSymbol(number, lines){
			total += valueOf(number, lines)
		}
	}
	fmt.Println("Part One:", total)
}

func isGearSymbol(c Coordinate, g Graph)(bool){
	v := rune(g[c.row][c.col])
	if v == '*' {
		return true
	} else {
		return false
	}
}

func adjascentGears(n PartNumber, g Graph)([]Coordinate){
	gears := []Coordinate{}
	for _,coord := range getNumberBoundary(n,g) {
		if coord.row < 0 || coord.col < 0 {
			continue
		}
		if coord.row >= len(g) || coord.col >= len(g[0]){
			continue
		}
		if isGearSymbol(coord, g){
			gears = append(gears, coord)
		}
	}
	return gears
}

func PartTwo(lines []string){

	numbers := getNumbers(lines)
	gears := map[Coordinate][]PartNumber{}

	for _,number := range numbers {
		adjascent := adjascentGears(number, lines)
		for _,g := range adjascent {
			_, ok := gears[g]
			if ok {
				gears[g] = append(gears[g], number)
			} else {
				gears[g] = []PartNumber{ number }
			}
		}
	}
	total := 0
	
	for _,nums :=range gears {
		if len(nums) == 2 {
			total += valueOf(nums[0], lines) * valueOf(nums[1], lines)
		}
	}
	fmt.Println("Part Two:", total)
}

func main() {
	input := shared.ReadLines()

	PartOne(input)
	PartTwo(input)
}