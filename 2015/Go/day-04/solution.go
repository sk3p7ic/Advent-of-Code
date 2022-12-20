package main;

import (
  "fmt"
  "math"
  "crypto/md5"
  "encoding/hex"
);

func doHashing(input *string, target uint8) uint32 {
  for i := 0; i < math.MaxUint32; i++ {
    var key = *input + fmt.Sprint(i);
    var bkey = []byte(key);
    var hasher = md5.New();
    hasher.Write(bkey);
    hash := hasher.Sum(nil);
    var hashString string = hex.EncodeToString(hash);
    for j, c := range hashString {
      if c != '0' {
        break;
      }
      if uint8(j) == target - 1 { // Because j + 1 == 5
        return uint32(i);
      }
    }
  }
  return 0;
}

func main() {
  var data string = "bgvyzdsv";
  p1 := doHashing(&data, 5);
  fmt.Printf("Day 04, Part 01: %d\n", p1);
  p2 := doHashing(&data, 6);
  fmt.Printf("Day 04, Part 02: %d\n", p2);
}
