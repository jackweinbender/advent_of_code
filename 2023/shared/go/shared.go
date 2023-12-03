package shared

import (
	"fmt"
	"strconv"
)

func Testing() {
	fmt.Print("Testing")
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