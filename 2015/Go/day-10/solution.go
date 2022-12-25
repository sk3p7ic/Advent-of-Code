package main

import (
	"fmt"
	"strconv"
)

type Count struct {
	num uint8
	occ uint16
}

func do_round(data *string) string {
	var counts []Count
	var ptr = 0
	for ptr < len(*data) {
		var num = (*data)[ptr]
    var occ uint8 = 0
    for ptr < len(*data) && (*data)[ptr] == num {
      occ++
      ptr++
    }
    counts = append(counts, Count{num: uint8(num), occ: uint16(occ)})
	}
  var new_string = ""
  for _, count := range counts {
    new_string += strconv.Itoa(int(count.occ)) + strconv.Itoa(int(count.num))
  }
  return new_string
}

func main() {
	var data = "1113122113"
  var p1_data = data
  var p2_data = data
  for i := 0; i < 40; i++ {
    p1_data = do_round(&p1_data)
  }
  for i := 0; i < 50; i++ {
    p2_data = do_round(&p2_data)
  }
  fmt.Printf("Day 10, Part 01: %d\n", len(p1_data))
  fmt.Printf("Day 10, Part 02: %d\n", len(p2_data))
}
