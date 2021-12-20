package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("data/diagnostic_sample.txt")

	if err != nil {
		panic(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	matrix := [][]int{}

	i := 0
	for scanner.Scan() {
		vals := strings.Split(scanner.Text(), "")

		matrix = append(matrix, []int{})

		for _, val := range vals {
			iVal, err := strconv.Atoi(val)

			if err != nil {
				panic(err)
			}

			matrix[i] = append(matrix[i], iVal)
		}
		i++
	}

	h := len(matrix)
	w := len(matrix[0])

	for i := 0; i < w; i++ {
		colTotal := 0

		for _, row := range matrix {
			colTotal += row[i]
		}

		filter := 1

		if colTotal <= (h / 2) {
			filter = 0
		}

		prevResult := matrix
		matrix = [][]int{}

		for _, row := range prevResult {
			if row[i] == filter {
				matrix = append(matrix, row)
			}
		}

		h = len(matrix)
	}

	fmt.Println(matrix[0])
}
