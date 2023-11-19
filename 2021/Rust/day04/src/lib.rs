#[derive(Copy, Clone, Debug)]
pub struct BingoValue {
    value: u8,
    called: bool
}

#[derive(Copy, Clone, Debug)]
pub struct BingoBoard {
    pub nums: [BingoValue; 25],
}

impl BingoBoard {
    pub fn init(values: &Vec<u8>) -> Option<Self> {
        if values.len() != 25 {
            return None;
        }
        let mut nums: [BingoValue; 25] = [BingoValue{value: 0, called: false}; 25];
        for i in 0..nums.len() {
            nums[i].value = *values.get(i).unwrap();
        }
        Some(Self{nums})
    }

    pub fn play(self, number: u8) -> Self {
        for mut n in self.nums {
            if n.value == number {
                n.called = true;
            }
        }
        self
    }

    pub fn n_to_win(&mut self, calls: &Vec<u8>) -> usize {
        let mut n_to_win_count = 0;
        for c in calls {
            n_to_win_count += 1;
            let mut idx = 0;
            for mut n in self.nums {
                if n.value == *c {
                    //n.called = true;
                    n = BingoValue{value: n.value, called: true};
                }
                self.nums[idx] = n;
                idx += 1;
            }
            if self.is_winner() {
                break;
            }
        }
        dbg!(self.nums);
        return n_to_win_count;
    }

    pub fn is_winner(&self) -> bool {
        // Check rows
        for r in 0..5 {
            let mut winner = false;
            for c in 0..5 {
                winner = self.nums[r*5+c].called;
                if !winner {
                    break;
                }
            }
            if winner {
                return true;
            }
        }
        // Check cols
        for c in 0..5 {
            let mut winner = false;
            for r in 0..5 {
                winner = self.nums[r*5+c].called;
                if !winner {
                    break;
                }
            }
            if winner {
                return true;
            }
        }
        return false;
    }
}

pub fn process_p1(input: &str) -> u16 {
    let parsed = parse_input(input);
    let called_nums = parsed.0;
    let mut boards = parsed.1;
    //let mut winning_board: Option<&BingoBoard> = None;
    //let mut winning_number = 0;
    //for cn in called_nums {
    //    boards = boards.iter()
    //        .map(|b| b.play(cn))
    //        .collect();
    //    dbg!(&boards);
    //    if boards.iter().any(|b| b.is_winner()) {
    //        winning_board = Some(boards.iter_mut()
    //            .find(|b| b.is_winner())
    //            .expect("there to be a winning board"));
    //        winning_number = cn;
    //        break;
    //    }
    //}
    //match winning_board {
    //    Some(b) => b.nums.iter()
    //        .filter(|n| !n.called)
    //        .map(|n| u16::from(n.value))
    //        .sum::<u16>() * winning_number as u16,
    //    None => 0
    //}
    let win_mins = boards.iter_mut()
        .map(|b| b.n_to_win(&called_nums))
        .collect::<Vec<usize>>();
    dbg!(&win_mins);
    let mut min_index = 0;
    let mut curr_idx = 0;
    let mut curr_min = usize::MAX;
    for wm in win_mins {
        if wm < curr_min {
            curr_min = wm;
            min_index = curr_idx;
        }
        curr_idx += 1;
    }
    dbg!(min_index);
    let winning_board = boards.get(min_index);
    let winning_number = called_nums.get(min_index).expect("number to exist");
    match winning_board {
        Some(b) => b.nums.iter()
            .filter(|n| !n.called)
            .map(|n| u16::from(n.value))
            .sum::<u16>() * *winning_number as u16,
        None => 0
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoBoard>) {
    let mut objects = input.trim().split("\n\n");
    let called_nums_str = objects.next().expect("there to be called numbers");
    let called_nums = called_nums_str.trim().split(",")
        .map(|s| s.parse::<u8>().expect("value to be a u8"))
        .collect::<Vec<u8>>();
    let boards = objects
        .map(|obj| obj.trim().replace("\n", " ")
            .trim()
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<u8>().expect("value to be a u8"))
            .collect::<Vec<u8>>())
        .map(|v| BingoBoard::init(&v).expect("board to be valid"))
        .collect::<Vec<BingoBoard>>();
    (called_nums, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7";

    #[test]
    fn p1_works() {
        assert_eq!(process_p1(&SAMPLE), 4512);
    }
}
