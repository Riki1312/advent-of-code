package day1

import (
	"aoc/2021/utils"
	"fmt"
	"strconv"

	lo "github.com/samber/lo"
)

func RunB() {
	lines, err := utils.FileToLines("./day1/input.txt")
	if err != nil {
		fmt.Println(err)
	}

	values := lo.Map(lines, func(x string, i int) int {
		r, err := strconv.Atoi(x)
		if err != nil {
			fmt.Println(err)
		}
		return r
	})

	var pred = values[0] + values[1] + values[2]
	var succ = values[1] + values[2] + values[3]
	var incr = 0

	for i := range values {
		if succ-pred > 0 {
			incr++
		}

		indx := i + 2
		if indx+2 >= len(values) {
			break
		}

		pred = succ
		succ = values[indx] + values[indx+1] + values[indx+2]
	}

	fmt.Println(incr)
}
