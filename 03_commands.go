package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("data/commands.txt")

	if err != nil {
		panic(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var depth int
	var x int

	for scanner.Scan() {
		vals := strings.Split(scanner.Text(), " ")
		command := vals[0]
		val := vals[1]

		ival, err := strconv.Atoi(val)

		if err != nil {
			panic(err)
		}

		switch command {
		case "forward":
			x = x + ival
		case "down":
			depth = depth + ival
		case "up":
			depth = depth - ival
		}
	}

	fmt.Println(depth * x)
}
