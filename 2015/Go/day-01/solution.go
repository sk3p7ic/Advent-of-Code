package main

import (
	"fmt"
	"io/ioutil"
  "strings"
)

func doP1(data string) uint16 {
  return uint16(strings.Count(data, "(")) - uint16(strings.Count(data, ")"));
}

func doP2(data string) uint16 {
  var f int16 = 0;
  for i := 0; i < len(data); i++ {
    c := string(data[i]);
    if c == "(" {
      f++;
    } else {
      f--;
    }
    if f == -1 {
      return uint16(i) + 1;
    }
  }
  return 0;
}

func main() {
  body, err := ioutil.ReadFile("input.txt");
  if err != nil {
    panic("Could not real file!");
  }
  var data = string(body);
  p1 := doP1(data);
  fmt.Printf("Day 01, Part 01: %d\n", p1);
  p2 := doP2(data);
  fmt.Printf("Day 01, Part 02: %d\n", p2);
}
