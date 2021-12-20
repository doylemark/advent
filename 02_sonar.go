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

	scanner := bufio.NewScanner(f)

	var values []int

	for scanner.Scan() {
		text := scanner.Text()

		val, err := strconv.Atoi(text)

		if err != nil {
			panic(err)
		}

		values = append(values, val)
	}

	var prev int
	var nIncreases int

	for i := range values {
		if i+3 <= len(values) {
			var total int

			for _, val := range values[i : i+3] {
				total = total + val
			}

			if total > prev && prev != 0 {
				nIncreases++
			}

			prev = total
		}
	}

	fmt.Println(nIncreases)
}
