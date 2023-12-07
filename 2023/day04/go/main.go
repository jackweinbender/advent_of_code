package main

import (
	"fmt"
	"math"
	"shared"
	"sort"
	"strings"
)

type Card struct {
	id int
	left []int
	right []int
}

func partition(s []string)([]int, []int){
	pivot := false
	
	left := []int{}
	right := []int{}

	for _,word := range s {
		if word == "|" {
			pivot = true
			continue
		}

		num,_ := shared.ParseInt(word)
		if pivot {
			right = append(right, int(num))
		} else {
			left = append(left, int(num))
		}
	}
	return left, right
}

func intersection(l []int, r []int)([]int){
	sort.Sort(sort.IntSlice(l))
	sort.Sort(sort.IntSlice(r))
	n := []int{}
	for i,j := 0,0; i<len(l) && j<len(r); {
		if l[i] == r[j] {
			n = append(n, l[i])
		}
		if l[i] <= r[j] {
			i++
		} else if l[i] >= r[j] {
			j++
		}
	}
	return n
}

func scoreCard(c Card)(int){
	n := matches(c)
	if n == 0 {
		return 0
	} else {
		return int(math.Pow(2, float64(n-1)))
	}
}

func matches(c Card)(int){
	n := intersection(c.left, c.right)
	return len(n)
}

func parseLine(line string)(Card){
	fields := strings.Fields(line)

	id,_ := shared.ParseInt(fields[1][0:len(fields[1])-1])
	left, right := partition(fields[2:])

	return Card{
		id: int(id),
		left: left,
		right: right,
	}
}

func rackEm(lines []string)([]Card){
	stack := []Card{}
	for _,line := range lines {
		stack = append(stack, parseLine(line))
	}
	return stack
}

func PartOne(lines []string)(any){
	total := 0
	stack := rackEm(lines)
	
	for _,card := range stack {
		total += scoreCard(card)
	}
	return total
}

func PartTwo(lines []string)(any){
	total := 0
	cards := rackEm(lines)
	// and extra copy. Shrug.
	queue := rackEm(lines)
	for len(queue) > 0 {
		pop := queue[0]
		queue = queue[1:]
		count := matches(pop)
		
		if count > 0 {
			idx := pop.id
			queue = append(queue, cards[idx:idx + count]...)
		}
		total++
	}

	return total
}

func main() {
	input := shared.ReadLines()
	fmt.Println("Part One:", PartOne(input))
	fmt.Println("Part Two:", PartTwo(input))
}