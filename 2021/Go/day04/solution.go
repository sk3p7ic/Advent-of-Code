package main;

import (
    "strings"
    "strconv"
    "os"
);

type BoardValue struct {
    value uint8;
    called bool;
}

type Board struct {
    board [25]BoardValue;
}

func (b *Board) Init(values [25]uint8) {
    for i := 0; i < 25; i++ {
        b.board[i].value = values[i];
        b.board[i].called = false;
    }
}

func (b *Board) Play(value uint8) {
    for i := 0; i < 25; i++ {
        if b.board[i].value == value {
            b.board[i].called = true;
        }
    }
}

func (b *Board) IsWinner() bool {
    // Check by row
    for r := 0; r < 5; r++ {
        for c := 0; c < 5; c++ {
            if b.board[r * 5 + c].called == false {
                break;
            }
            if c == 4 {
                return true;
            }
        }
    }
    // Check by column
    for c := 0; c < 5; c++ {
        for r := 0; r < 5; r++ {
            if b.board[r * 5 + c].called == false {
                break;
            }
            if r == 4 {
                return true;
            }
        }
    }
    return false;
}

func (b *Board) GetScore(winning_num uint8) uint16 {
    var score uint16 = 0;
    for i := 0; i < 25; i++ {
        if b.board[i].called == false {
            score += uint16(b.board[i].value);
        }
    }
    return score * uint16(winning_num);
}

func (b *Board) PrintBoard() {
    for r := 0; r < 5; r++ {
        for c := 0; c < 5; c++ {
            if b.board[r * 5 + c].called {
                os.Stdout.WriteString("X ");
            } else {
                os.Stdout.WriteString(strconv.Itoa(int(b.board[r * 5 + c].value)) + " ");
            }
        }
        os.Stdout.WriteString("\n");
    }
}

type Game struct {
    called_numbers []uint8;
    boards []Board;
}

func (g *Game) PlayForPart1() (winning_num uint8, board *Board) {
    for round_num, cn := range g.called_numbers {
        for i, b := range g.boards {
            b.Play(cn);
            g.boards[i] = b;
        }
        if round_num >= 5 {
            for _, b := range g.boards {
                if b.IsWinner() {
                    return cn, &b;
                }
            }
        }
    }
    return 0, nil;
}

func (g *Game) PlayForPart2() (winning_num uint8, board *Board) {
    for round_num, cn := range g.called_numbers {
        for i, b := range g.boards {
            b.Play(cn);
            g.boards[i] = b;
        }
        if round_num >= 5 {
            // Remove boards that are winners
            var prev_len int = len(g.boards);
            for { // Loop until no more winners are found or only one board is left
                for i, b := range g.boards {
                    if b.IsWinner() {
                        g.boards = append(g.boards[:i], g.boards[i+1:]...);
                        break;
                    }
                }
                if len(g.boards) == 1 || len(g.boards) == prev_len {
                    break;
                }
                prev_len = len(g.boards);
            }
        }
        if len(g.boards) == 1 {
            var next_num uint8 = g.called_numbers[round_num+1];
            for i, b := range g.boards {
                b.Play(next_num);
                g.boards[i] = b;
            }
            return next_num, &g.boards[0];
        }
    }
    return 0, nil;
}

func ParseInput(input string) (Game, error) {
    var g Game;
    g.boards = make([]Board, 0);
    g.called_numbers = make([]uint8, 0);
    input_sections := strings.Split(input, "\n\n");
    called_nums_str := strings.Split(input_sections[0], ",");
    boards_strs := input_sections[1:];
    for _, cns := range called_nums_str {
        if cns == "" {
            continue;
        }
        cn, err := strconv.Atoi(cns);
        if err != nil {
            return g, err;
        }
        g.called_numbers = append(g.called_numbers, uint8(cn));
    }
    for _, bs := range boards_strs {
        var b Board;
        var values [25]uint8;
        bs = strings.Replace(bs, "\n", " ", -1);
        bsa := strings.Split(bs, " ");
        var offset int = 0; // For empty strings
        for i, v := range bsa {
            if v == "" {
                offset++;
                continue;
            }
            bv, err := strconv.Atoi(v);
            if err != nil {
                return g, err;
            }
            values[i-offset] = uint8(bv);
        }
        b.Init(values);
        g.boards = append(g.boards, b);
    }
    return g, nil;
}

func main() {
    input, err := os.ReadFile("input.txt");
    if err != nil { panic(err) }
    g, err := ParseInput(string(input));
    if err != nil { panic(err) }
    winning_num, board := g.PlayForPart1();
    if board == nil {
        panic("No winner found");
    }
    println("D4P1:", board.GetScore(winning_num));
    g, err = ParseInput(string(input));
    if err != nil { panic(err) }
    winning_num, board = g.PlayForPart2();
    if board == nil {
        panic("No winner found");
    }
    println("D4P2:", board.GetScore(winning_num));
}
