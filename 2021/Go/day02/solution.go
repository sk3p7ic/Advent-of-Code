package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type CommandDirection int8

const (
    Forward CommandDirection = 0
    Down    CommandDirection = 1
    Up      CommandDirection = -1
)

type Command struct {
    Direction CommandDirection
    Value int
}

type Coords struct {
    X, Y, A int
}

func p1(commands *[]Command) int {
    var pos Coords = Coords{0, 0, 0}
    for i := 0; i < len(*commands); i++ {
        switch (*commands)[i].Direction {
        case Forward:
            pos.X += (*commands)[i].Value
        case Down:
            pos.Y += (*commands)[i].Value
        case Up:
            pos.Y -= (*commands)[i].Value
        }
    }
    return pos.X * pos.Y;
}

func p2(commands *[]Command) int {
    var pos Coords = Coords{0, 0, 0}
    for i := 0; i < len(*commands); i++ {
        switch (*commands)[i].Direction {
        case Forward:
            pos.X += (*commands)[i].Value
            pos.Y += (*commands)[i].Value * pos.A
        case Down:
            pos.A += int(Down) * (*commands)[i].Value
        case Up:
            pos.A += int(Up) * (*commands)[i].Value
        }
    }
    return pos.X * pos.Y;
}

func get_commands() []Command {
    text, err := os.ReadFile("./input.txt")
    if err != nil {
        fmt.Println("Could not read file.")
        os.Exit(1)
    }
    var lines = strings.Split(string(text[:]), "\n")
    var commands []Command
    for i := 0; i < len(lines); i++ {
        if len(lines[i]) == 0 {
            continue
        }
        var line = strings.Split(lines[i], " ")
        var direction CommandDirection
        switch dirStr := line[0]; dirStr {
        case "forward":
            direction = Forward
        case "down":
            direction = Down
        case "up":
            direction = Up
        }
        val, err := strconv.Atoi(line[1])
        if err != nil {
            fmt.Println("Could not parse int: ", line[1])
        }
        commands = append(commands, Command{direction, val})
    }
    return commands
}

func main() {
    commands := get_commands()
    fmt.Println("D1P1: ", p1(&commands))
    fmt.Println("D1P2: ", p2(&commands))
}
