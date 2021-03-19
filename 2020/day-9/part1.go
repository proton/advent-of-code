package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func readNumbers(filepath string) (numbers []int) {
	content, _ := ioutil.ReadFile(filepath)

	lines := strings.Split(string(content), "\n")
	numbers = make([]int, 0, len(lines))

	for _, l := range lines {
		n, _ := strconv.Atoi(l)
		numbers = append(numbers, n)
	}

	return numbers
}

func main() {
	numbers := readNumbers("input.txt")

	previousNumbersCount := 25

	for i, n := range numbers[previousNumbersCount:] {
		previousNumbers := numbers[i : i+previousNumbersCount]

		isOk := false
		for x, p1 := range previousNumbers {
			for y, p2 := range previousNumbers {
				if isOk {
					break
				}
				if x == y {
					continue
				}
				isOk = (p1+p2 == n)
			}
		}
		if !isOk {
			fmt.Println(n)
		}
	}
}
