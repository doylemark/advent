package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.Open("data/diagnostic.txt")

	if err != nil {
		panic(err)
	}

	defer f.Close()

	scanner := bufio.NewScanner(f)

	type AvgRecord map[int]int

	var bitAvgs []AvgRecord

	for scanner.Scan() {
		vals := strings.Split(scanner.Text(), "")

		for i, val := range vals {
			ival, err := strconv.Atoi(val)

			if err != nil {
				panic(err)
			}

			if len(bitAvgs) <= i {
				avg := AvgRecord{
					0: 0,
					1: 0,
				}
				avg[ival]++

				bitAvgs = append(bitAvgs, avg)
			} else {
				bitAvgs[i][ival]++
			}
		}
	}

	var gamma string
	var epsilon string

	for _, avg := range bitAvgs {
		if avg[0] > avg[1] {
			gamma += "0"
			epsilon += "1"
		} else {
			epsilon += "0"
			gamma += "1"
		}
	}

	igamma, err := strconv.ParseInt(gamma, 2, 64)

	if err != nil {
		panic(err)
	}

	iepsilon, err := strconv.ParseInt(epsilon, 2, 64)

	if err != nil {
		panic(err)
	}

	power := igamma * iepsilon

	fmt.Println(power)
}
