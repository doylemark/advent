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

	scanner := bufio.NewScanner(f)

	var depth int
	var x int
	var depthVector int

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
			depth = depth + (ival * depthVector)
			x = x + ival
		case "down":
			depthVector = depthVector + ival
		case "up":
			depthVector = depthVector - ival
		}
	}

	fmt.Println(depth * x)
}
