package shared

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func ReadLines() []string {
	scanner := bufio.NewScanner(os.Stdin)
	var input []string
	for scanner.Scan() {
		input = append(input, scanner.Text())
	}
	return input
}

func ParseInt(s string)(int64, error){
	num, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		// So I don't have to deal with logging these every time
		// and I can be lazyin my code.
		fmt.Println("Failed to parse", s, "to integer,", err)
	}
	return num, err
}