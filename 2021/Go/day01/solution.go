package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func p1(depths *[]int) int {
    var n_incr = 0
    for i := 1; i < len(*depths); i++ {
        if (*depths)[i] > (*depths)[i-1] {
            n_incr += 1
        }
    }
    return n_incr
}

func p2(depths *[]int) int {
    var n_incr = 0;
    var sums []int
    for i := 0; i < len(*depths) - 2; i++ {
        sums = append(sums, (*depths)[i] + (*depths)[i+1] + (*depths)[i+2])
    }
    for i := 1; i < len(sums); i++ {
        if sums[i] > sums[i-1] {
            n_incr += 1
        }
    }
    return n_incr;
}

func get_depths() []int {
    text, err := os.ReadFile("./input.txt")
    if err != nil {
        fmt.Println("Could not read file.")
        os.Exit(1)
    }
    var lines = strings.Split(string(text[:]), "\n")
    var depths []int
    for i := 0; i < len(lines); i++ {
        if len(lines[i]) == 0 {
            continue
        }
        d, e := strconv.Atoi(lines[i])
        if e != nil {
            fmt.Println(e)
            os.Exit(1)
        }
        depths = append(depths, d)
    }
    return depths
}

func main() {
    depths := get_depths()
    fmt.Println("D1P1: ", p1(&depths))
    fmt.Println("D1P2: ", p2(&depths))
}
