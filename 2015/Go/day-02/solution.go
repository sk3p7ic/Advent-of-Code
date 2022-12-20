package main

import (
	"fmt"
	"io/ioutil"
	"strings"
  "strconv"
)

func getMin(vals *[]uint32) uint32 {
  var min uint32 = 0;
  for i, v := range *vals {
    if i == 0 || v < min {
      min = v;
    }
  }
  return min;
}

func doP1(dimens *[][]uint8) uint32 {
  var total uint32 = 0;
  for _, d := range *dimens {
    l, h, w := uint32(d[0]), uint32(d[1]), uint32(d[2]);
    var sides = []uint32{l * w, l * h, w * h};
    for _, s := range sides {
      total += (2 * s);
    }
    total += getMin(&sides);
  }
  return total;
}


func doP2(dimens *[][]uint8) uint32 {
  var total uint32 = 0;
  for _, d := range *dimens {
    l, h, w := uint32(d[0]), uint32(d[1]), uint32(d[2]);
    var perims = []uint32{2 * l + 2 * h, 2 * l + 2 * w, 2 * w + 2 * h};
    total += getMin(&perims) + (l * w * h);
  }
  return total;
}


func main() {
  body, err := ioutil.ReadFile("input.txt");
  if err != nil {
    panic("Could not read file!")
  }
  var data = strings.Split(string(body), "\n");
  var dimens [][]uint8;
  for i := 0; i < len(data); i++ {
    var line string = data[i];
    ds := strings.Split(line, "x");
    if len(ds) != 3 {
      continue;
    }
    var box = []uint8{};
    for _, s := range ds {
      si, err := strconv.Atoi(s);
      if err != nil {
        panic("Could not properly cast value!");
      }
      box = append(box, uint8(si));
    }
    dimens = append(dimens, box);
  }
  p1 := doP1(&dimens);
  fmt.Printf("Day 02, Part 01: %d\n", p1);
  p2 := doP2(&dimens);
  fmt.Printf("Day 02, Part 02: %d\n", p2);
}
