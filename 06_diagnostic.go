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

	w := len(matrix[0])

	// iterate over each column in the matrix
	for i := 0; i < w; i++ {
		maj := findMajVal(matrix, i)

		prev := matrix
		matrix = [][]int{}

		for _, row := range prev {
			if row[i] == maj {
				matrix = append(matrix, row)
			}
		}

		fmt.Println(matrix)
	}
}

func findMajVal(matrix [][]int, colIndex int) int {
	count := 0

	for _, row := range matrix {
		count = count + row[colIndex]
	}

	if count >= (len(matrix) / 2) {
		return 1
	} else {
		return 0
	}
}
