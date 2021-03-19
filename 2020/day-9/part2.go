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

func minMax(array []int) (int, int) {
	var max int = array[0]
	var min int = array[0]
	for _, value := range array {
		if max < value {
			max = value
		}
		if min > value {
			min = value
		}
	}
	return min, max
}

func findRange(numbers []int, mainNumber int) (ran []int) {
	for i, n := range numbers {
		s := n
		for j, m := range numbers[i:] {
			if j == 0 {
				continue
			}
			s += m
			if s > mainNumber {
				break
			}
			if s == mainNumber {
				return numbers[i : i+j+1]
			}
		}
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
			min, max := minMax(findRange(numbers, n))
			fmt.Println(min + max)
		}
	}
}
