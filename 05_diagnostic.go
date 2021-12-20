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

	colTotals := make([]int, len(matrix[0]))

	for _, row := range matrix {
		for i, val := range row {
			colTotals[i] += val
		}
	}

	var g string
	var e string

	for _, col := range colTotals {
		if col >= (len(matrix) / 2) {
			g += "1"
			e += "0"
		} else {
			g += "0"
			e += "1"
		}
	}

	dg, err := strconv.ParseInt(g, 2, 64)

	if err != nil {
		panic(err)
	}

	de, err := strconv.ParseInt(e, 2, 64)

	if err != nil {
		panic(err)
	}

	fmt.Println(dg, dg*de)
}
