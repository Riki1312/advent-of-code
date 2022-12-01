package day1

import (
	"aoc/go-2021/utils"
	"fmt"
	"strconv"

	lo "github.com/samber/lo"
)

func RunA() {
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

	var pred = values[0]
	var succ = values[1]
	var incr = 0

	for _, v := range values {
		if succ-pred > 0 {
			incr++
		}

		pred = succ
		succ = v
	}

	fmt.Println(incr)
}
