package main

import (
	"bufio"
	"fmt"
	"os"
	"shared"
	"strings"
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

type Round map[string]int

type Game struct {
	id int
	rounds []Round
}

func ParseGame(input string)(Game){
	arr := strings.Split(input, ": ")
	gamestr := strings.Split(arr[0], " ")
	roundsstr := strings.Split(arr[1], "; ")

	gameId, _ := shared.ParseInt(gamestr[1])

	game := Game{
		id: int(gameId),
		rounds: []Round{},
	}

	for _,round := range roundsstr {
		game.rounds = append(game.rounds, ParseRound(round))
	}

	return game
}
func ParseRound(input string)(Round){
	// 3 blue, 4 red (comma separated)
	colors := strings.Split(input, ", ")
	
	round := Round{
		"blue": 0,
		"green": 0,
		"red": 0,
	}

	for _,str := range colors {
		arr := strings.Split(str, " ")
		color := arr[1]
		count, _ := shared.ParseInt(arr[0])

		round[color] = int(count)
	}
	return round
}

func isValidRound(round Round, red int, green int, blue int)(bool){
	return round["red"] <= red && round["green"] <= green && round["blue"] <= blue 
}

func isValidGame(game Game, red int, green int, blue int)(bool){
	for _,round := range game.rounds {
		if !isValidRound(round, red, green, blue){
			return false
		}
	}
	return true
}

func PartOne(lines []string){
	const red = 12
	const green = 13
	const blue = 14

	total := 0

	for _,line := range lines {
		game := ParseGame(line)
		if isValidGame(game, red, green, blue){
			total += game.id
		}
	}
	fmt.Println("Part One:", total)
}

func fewestCubes(game Game)(Round){
	minimalRound := Round{
		"blue": 0,
		"green": 0,
		"red": 0,
	}

	for _,round := range game.rounds {
		minimalRound = updateMin(minimalRound, round)
	}

	return minimalRound
}

func updateMin(base Round, new Round)(Round){
	return Round{
		"blue": max(base["blue"], new["blue"]),
		"green": max(base["green"], new["green"]),
		"red": max(base["red"], new["red"]),
	}
}

func powerForRound(round Round)(int){
	return round["red"] * round["green"] * round["blue"]
}

func PartTwo(lines []string) {
	const red = 12
	const green = 13
	const blue = 14

	total := 0

	for _,line := range lines {
		game := ParseGame(line)
		minimalSet := fewestCubes(game)
	
		total += powerForRound(minimalSet)
	}
	fmt.Println("Part Two:", total)
}
