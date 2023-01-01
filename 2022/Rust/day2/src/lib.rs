enum Moves {
    Rock, Paper, Scissors
}

impl Moves {
    fn to_move_p1(input: &str) -> Moves {
        match input {
            "A" => Moves::Rock,
            "B" => Moves::Paper,
            "C" => Moves::Scissors,
            "X" => Moves::Rock,
            "Y" => Moves::Paper,
            "Z" => Moves::Scissors,
            _ => panic!("Invalid type.")
        }
    }

    fn to_num(&self) -> u16 {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    fn get_kryptonite(&self) -> Moves {
        match *self {
            self::Moves::Rock => Self::Paper,
            self::Moves::Paper => Self::Scissors,
            self::Moves::Scissors => Self::Rock
        }
    }

    fn get_a_loser(&self) -> Moves {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper
        }
    }
}

enum Outcomes {
    Lose, Draw, Win
}

impl Outcomes {
    fn to_outcome(input: &char) -> Outcomes {
        match input {
            'X' => Outcomes::Lose,
            'Y' => Outcomes::Draw,
            'Z' => Outcomes::Win,
            _ => panic!("Ivalid type.")
        }
    }

    fn calc_score(&self, hand: &Moves) -> u16 {
        match self {
            Self::Lose => hand.get_a_loser().to_num(),
            Self::Draw => hand.to_num() + 3,
            Self::Win => hand.get_kryptonite().to_num() + 6
        }
    }
}

pub fn run_p1(input: &str) -> u16 {
    let parsed = input.split("\n").collect::<Vec<&str>>();
    let mut score = 0u16;
    for line in parsed.into_iter() {
        if line.len() == 0 {
            continue;
        }
        let opp = Moves::to_move_p1(&line.chars().nth(0).unwrap().to_string());
        let sel = Moves::to_move_p1(&line.chars().nth(2).unwrap().to_string());
        if opp.to_num() == sel.get_kryptonite().to_num() {
            score += sel.to_num();
        }
        else if opp.to_num() == sel.to_num() {
            score += sel.to_num() + 3;
        }
        else {
            score += sel.to_num() + 6;
        }

    }
    score
}

pub fn run_p2(input: &str) -> u16 {
    let parsed = input.split("\n").collect::<Vec<&str>>();
    let mut score = 0u16;
    for line in parsed.into_iter() {
        if line.len() == 0 {
            continue;
        }
        let opp = Moves::to_move_p1(&line.chars().nth(0).unwrap().to_string());
        let us = Outcomes::to_outcome(&line.chars().nth(2).unwrap());
        score += us.calc_score(&opp);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("./../example.txt");

    #[test]
    fn test_p1() {
        let score = run_p1(&TEST_INPUT);
        assert_eq!(score, 15u16);
    }

    #[test]
    fn test_p2() {
        let score = run_p2(&TEST_INPUT);
        assert_eq!(score, 12);
    }
}
