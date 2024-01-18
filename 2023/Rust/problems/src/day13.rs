struct ProblemInput {
    lines: Vec<String>
}

impl std::str::FromStr for ProblemInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self{ lines: s.trim()
            .lines().map(|line| line.trim().to_string())
            .collect() })
    }
}

impl ProblemInput {

    fn to_col_string(&self, col: usize) -> String {
        self.lines.iter()
            .map(|line| line.chars().nth(col).expect("column to exist"))
            .map(|c| c.to_string())
            .collect()
    }

    fn check_vertical(&self) -> Option<usize> {
        if self.to_col_string(0) == self.to_col_string(1) {
            return Some(1);
        }
        let llen = self.lines[0].len();
        if self.to_col_string(llen-2) == self.to_col_string(llen-1) {
            return Some(llen-1);
        }
        for col in 0..llen - 3 {
            let strs: Vec<String> = vec![
                self.to_col_string(col),
                self.to_col_string(col + 1),
                self.to_col_string(col + 2),
                self.to_col_string(col + 3)
            ];
            //dbg!(&strs);
            // Doesn't currently account for if the symmetry is at the
            // beginning of a line
            if strs[0] == strs[3] && strs[1] == strs[2] {
                dbg!(&strs[2]);
                return Some(col + 2);
            }
        }
        None
    }

    fn check_horizontal(&self) -> Option<usize> {
        if self.lines[0] == self.lines[1] {
            return Some(1);
        }
        if self.lines[self.lines.len()-2] == self.lines[self.lines.len()-1] {
            return Some(self.lines.len()-1);
        }
        for row in 0..self.lines.len() - 3 {
            //println!("{}\n{}\n{}\n{}\n\n", self.lines[row+0], self.lines[row+1], self.lines[row+2], self.lines[row+3]);
            if self.lines[row + 0] == self.lines[row + 3]
                && self.lines[row + 1] == self.lines[row + 2] {
                dbg!(&self.lines[row + 2]);
                return Some(row + 2);
            }
        }
        None
    }

    fn check(&self) -> usize {
        dbg!(&self.lines);
        let n = self.check_vertical().unwrap_or_else(||
            100 * self.check_horizontal().expect("to exist if vert does not"));
        dbg!(n);
        n
    }
}

fn process_p1(patterns: &Vec<ProblemInput>) -> usize {
    patterns.iter().map(|p| p.check()).sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day13.in.txt").expect("file to exist");
    let patterns: Vec<ProblemInput> = input.split("\n\n")
        .map(|pattern| pattern.parse().expect("to be of valid form"))
        .collect();
    println!("[2023] D13P01: {}", process_p1(&patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn p1_works() {
        let (t1, t2) = SAMPLE.split_once("\n\n").unwrap();
        let (s1, s2): (ProblemInput, ProblemInput) = (t1.parse().unwrap(), t2.parse().unwrap());
        assert_eq!(5, s1.check());
        assert_eq!(400, s2.check());
    }
}
