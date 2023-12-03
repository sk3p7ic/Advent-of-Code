const DIRS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1)
];


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MtxNum {
    x: usize,
    dx: usize,
    y: usize,
    v: u32,
    gear_pos: Option<(usize, usize)>
}


impl MtxNum {
    pub fn check_adj(&self, mtx: &Vec<Vec<char>>) -> bool {
        let mut has_adj_char = false;
        let y = self.y; // To avoid using `self.` further down
        for x in self.x..self.dx {
            for d in DIRS {
                let row_idx: isize = (y as isize) + d.1;
                if row_idx < 0 { continue; }
                if let Some(row) = mtx.get(row_idx as usize) {
                    let col_idx: isize = (x as isize) + d.0;
                    if col_idx < 0 { continue; }
                    match row.get(col_idx as usize) {
                        Some(c) => {
                            match c {
                                '0'..='9' | '.' => {},
                                _ => {
                                    has_adj_char = true;
                                }
                            }
                        },
                        None => {}
                    };
                }
            }
        }
        has_adj_char
    }

    pub fn find_gear_pos(&mut self, mtx: &Vec<Vec<char>>) {
        let y = self.y; // To avoid using `self.` further down
        for x in self.x..self.dx {
            for d in DIRS {
                let row_idx: isize = (y as isize) + d.1;
                if row_idx < 0 { continue; }
                if let Some(row) = mtx.get(row_idx as usize) {
                    let col_idx: isize = (x as isize) + d.0;
                    if col_idx < 0 { continue; }
                    match row.get(col_idx as usize) {
                        Some(c) => {
                            match c {
                                '*' => {
                                    self.gear_pos = Some((row_idx as usize, col_idx as usize))
                                },
                                _ => {}
                            }
                        },
                        None => {}
                    };
                }
            }
        }
    }
}


fn vectorize(input: &str) -> Vec<Vec<char>> {
    input.trim().lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}


fn replace_non_gears(mtx: Vec<Vec<char>>) -> Vec<Vec<char>> {
    mtx.into_iter()
        .map(|row| row.into_iter()
            .map(|c| match c {
                '0'..='9' | '*' | '.' => c,
                _ => '.'
            })
            .collect())
        .collect()
}


fn parse_input_mtx(mtx: &Vec<Vec<char>>) -> Vec<MtxNum> {
    let mut numbers = Vec::new();
    for (i, r) in mtx.iter().enumerate() {
        let mut c_num: Vec<char> = Vec::new();
        let mut start: usize = 0;
        for (j, c) in r.iter().enumerate() {
            if c.is_digit(10) {
                if c_num.len() == 0 {
                    start = j;
                }
                c_num.push(*c);
            }
            if (!c.is_digit(10) || j == r.len() - 1) && c_num.len() != 0 {
                let digit = c_num.iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("to be a valid digit.");
                c_num = Vec::new();
                numbers.push(MtxNum{x: start, dx: j, y: i, v: digit, gear_pos: None});
                start = 0;
            }
        }
    }
    numbers
}


fn process_p1(input: &str) -> u32 {
    let mut are_adj: Vec<u32> = Vec::new();
    let mtx = vectorize(&input);
    let numbers = parse_input_mtx(&mtx);
    for n in numbers.iter() {
        if n.check_adj(&mtx) {
            are_adj.push(n.v);
        }
    }
    are_adj.into_iter().sum()
}


fn process_p2(input: &str) -> u32 {
    let mut are_adj: Vec<MtxNum> = Vec::new();
    let mtx = replace_non_gears(vectorize(&input));
    let numbers = parse_input_mtx(&mtx);
    for n in numbers.iter() {
        if n.check_adj(&mtx) {
            are_adj.push(n.clone());
        }
    }
    are_adj.iter_mut().for_each(|n| n.find_gear_pos(&mtx));
    let mut samesies: Vec<u32> = Vec::new();
    let mut last: MtxNum = are_adj[0].clone();
    for (i, n) in are_adj.into_iter().enumerate() {
        if i == 0 { continue; }
        if let Some(last_pos) = last.gear_pos {
            if let Some(curr_pos) = n.gear_pos {
                if last_pos == curr_pos {
                    samesies.push(last.v * n.v);
                }
            }
        }
        last = n.clone();
    }
    samesies.into_iter().sum()
}


fn main() {
    let input = std::fs::read_to_string("./inputs/day03.in.txt").expect("file to exist");
    println!("[2023] D03P01: {}", process_p1(&input));
    println!("[2023] D03P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn p1_works() {
        assert_eq!(4361, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(467835, process_p2(&SAMPLE));
    }
}
