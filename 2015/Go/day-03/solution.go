package main;

import (
  "fmt"
  "io/ioutil"
)

type Coord struct {
  x int8;
  y int8;
}

func doP1(data string) uint16 {
  var visited = map[Coord]bool{};
  var c Coord;
  c.x, c.y = 0, 0;
  visited[c] = true;

  for i := 0; i < len(data); i++ {
    var d = rune(data[i]);
    if d == '^' {
      c.y++;
    } else if d == '>' {
      c.x++;
    } else if d == 'v' {
      c.y--;
    } else if d == '<' {
      c.x--;
    } else {
      continue;
    }
    visited[c] = true;
  }

  return uint16(len(visited));
}

func doP2(data string) uint16 {
  var visited = map[Coord]bool{};
  var s Coord;
  var r Coord;
  s.x, s.y = 0, 0;
  r.x, r.y = 0, 0;
  visited[s] = true;

  for i := 0; i < len(data); i++ {
    var d = rune(data[i]);
    var dx int8 = 0;
    var dy int8 = 0;
    if d == '^' {
      dy++;
    } else if d =='>' {
      dx++;
    } else if d == 'v' {
      dy--;
    } else if d == '<' {
      dx--;
    } else {
      continue;
    }

    if i % 2 == 0 {
      s.x += dx;
      s.y += dy;
      visited[s] = true;
    } else {
      r.x += dx;
      r.y += dy;
      visited[r] = true;
    }
  }
  return uint16(len(visited));
}

func main() {
  body, err := ioutil.ReadFile("input.txt");
  if err != nil {
    panic("Could not open input file.");
  }
  var data string = string(body);
  p1 := doP1(data);
  fmt.Printf("Day 03, Part 01: %d\n", p1);
  p2 := doP2(data);
  fmt.Printf("Day 03, Part 02: %d\n", p2);
}
