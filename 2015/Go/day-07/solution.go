package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

/*
Due to the example, I'd assumed that the wires would be initialized in such an
order that I would not need to worry about a wire input depending on another
wire which has not yet been initialized--I now this this assumption was wrong.
*/

func process_line(input *string, wires *map[string]uint16) uint16 {
  line := strings.Split(*input, " ");
  if len(line) == 1 {
    return (*wires)[line[0]];
  } else if len(line) == 2 {
    var val uint16 = (*wires)[line[0]];
    return ^val;
  } else{
    switch line[1] {
      case "AND":
        var lhs, rhs = (*wires)[line[0]], (*wires)[line[1]];
        return lhs & rhs;
      case "OR":
        var lhs, rhs = (*wires)[line[0]], (*wires)[line[1]];
        return lhs | rhs;
      case "LSHIFT":
        var lhs, rhs = (*wires)[line[0]], (*wires)[line[1]];
        return lhs << rhs;
      case "RSHIFT":
        var lhs, rhs = (*wires)[line[0]], (*wires)[line[1]];
        return lhs >> rhs;
    }
  }
  return 0;
}

func build_map(input *[]string) map[string]uint16 {
  wires := make(map[string]uint16);
  for i := 0; i < len(*input); i++ {
    line := strings.Split((*input)[i], " -> ");
    if len(line) <= 1 {
      continue;
    }
    var val, wire = line[0], line[1];
    if conv, err := strconv.Atoi(val); err == nil {
      wires[wire] = uint16(conv);
    } else {
      wires[wire] = process_line(&val, &wires);
    }
  }
  return wires;
}

func do_p1(input *[]string, target string) uint16 {
  var wires = build_map(input);
  fmt.Println(wires);
  return wires[target];
}

func main() {
  body, err := ioutil.ReadFile("input.txt");
  if err != nil {
    panic("Could not read input file.");
  }
  var data []string = strings.Split(string(body), "\n");
  p1 := do_p1(&data, "a");
  fmt.Printf("Day 07, Part 01: %d\n", p1);
}
