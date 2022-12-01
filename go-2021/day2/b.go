package day2

import (
	"aoc/go-2021/utils"
	"fmt"
	"strconv"
)

func RunB() {
	words, err := utils.FileToWords("./day2/input.txt")
	if err != nil {
		fmt.Println(err)
	}

	var position = 0
	var depth = 0
	var aim = 0

	for i := range words {
		if i+1 >= len(words) {
			break
		}
		if i%2 != 0 {
			continue
		}

		fmt.Println(words[i], words[i+1])

		move := words[i]
		value, err := strconv.Atoi(words[i+1])
		if err != nil {
			fmt.Println(err)
		}

		if move == "forward" {
			position += value
			depth += aim * value
		} else if move == "up" {
			aim -= value
		} else if move == "down" {
			aim += value
		}
	}

	fmt.Println(position * depth)
}
