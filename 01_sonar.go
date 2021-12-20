package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	f, err := os.Open("data/sonar.txt")

	if err != nil {
		panic(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	var prev int
	var nIncreases int

	for scanner.Scan() {
		text := scanner.Text()

		val, err := strconv.Atoi(text)

		if err != nil {
			panic(err)
		}

		if val > prev && prev != 0 {
			nIncreases++
		}

		prev = val
	}

	fmt.Println(nIncreases)
}
