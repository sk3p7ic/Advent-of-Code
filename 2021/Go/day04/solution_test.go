package main;

import "testing";

const SampleInput = `7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7`;

func TestBasicBoardOps(t *testing.T) {
    var b Board;
    b.Init([25]uint8{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
        18, 19, 20, 21, 22, 23, 24, 25});
    b.Play(1);
    b.Play(6);
    b.Play(11);
    b.Play(16);
    b.Play(21);
    if b.board[0].called != true {
        t.Error("b.board[0].called != true");
    }
    if !b.IsWinner() {
        t.Error("!b.IsWinner()");
    }
}

func TestSampleInputParsing(t *testing.T) {
    game, err := ParseInput(SampleInput);
    if err != nil {
        t.Fatalf("ParseInput() returned an error: %v", err);
    }
    if len(game.called_numbers) != 27 {
        t.Error("len(game.called_numbers) != 27");
    }
    if len(game.boards) != 3 {
        t.Error("len(game.boards) != 3");
    }
}

func TestPart1Game(t *testing.T) {
    game, err := ParseInput(SampleInput);
    if err != nil {
        t.Fatalf("ParseInput() returned an error: %v", err);
    }
    winning_num, board := game.PlayForPart1();
    if board == nil {
        t.Error("board == nil");
    }
    score := board.GetScore(winning_num);
    if score != 4512 {
        t.Errorf("score != 4512 (%v)", score);
    }
}

func TestPart2Game(t *testing.T) {
    game, err := ParseInput(SampleInput);
    if err != nil {
        t.Fatalf("ParseInput() returned an error: %v", err);
    }
    winning_num, board := game.PlayForPart2();
    if board == nil {
        t.Error("board == nil");
    }
    score := board.GetScore(winning_num);
    if score != 1924 {
        t.Errorf("score != 1924 (%v)", score);
    }
}
