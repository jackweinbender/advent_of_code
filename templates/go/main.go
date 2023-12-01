package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var input []string

	for scanner.Scan() {
		input = append(input, strings.Trim(scanner.Text()) )
	}

	fmt.Print(input)
}
