enum Feature { Plot, Rock }

impl std::str::FromStr for Feature {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0).expect("string to have first character") {
            '.' | 'S' => Ok(Self::Plot),
            '#' => Ok(Self::Rock),
            c => Err(format!("Unknown Landscape type '{}'.", c))
        }
    }
}

struct LandscapeMap {
    landscape: Vec<Vec<Feature>>,
    start_pos: (usize, usize)
}

impl std::str::FromStr for LandscapeMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let landscape: Vec<Vec<Feature>> = s.trim().lines()
            .map(|line| line.chars()
                .map(|c| c.to_string().as_str().parse::<Feature>().expect("to be a Feature"))
                .collect())
            .collect();
        let mut start_pos = (0, 0);
        s.trim().lines().enumerate()
            .for_each(|(row, ln)| ln.chars().enumerate()
                .for_each(|(col, c)| {
                    if c == 'S' { start_pos = (row, col); }
                }));
        Ok(Self { landscape, start_pos })
    }
}

impl LandscapeMap {
    fn check_surrounding(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let ipos = (pos.0 as isize, pos.1 as isize);
        let trans_mtx = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        let mut results: Vec<(usize, usize)> = Vec::new();
        for t in trans_mtx {
            if ipos.0 + t.0 >= 0 && ipos.0 + t.0 < self.landscape.len() as isize &&
                ipos.1 + t.1 >= 0 && ipos.1 + t.1 < self.landscape[0].len() as isize {
                let npos = ((ipos.0 + t.0) as usize, (ipos.1 + t.1) as usize);
                match self.landscape[npos.0][npos.1] {
                    Feature::Plot => results.push(npos),
                    Feature::Rock => {}
                }
            }
        }
        results
    }

    fn traverse(&mut self, n_steps: usize) -> u16 {
        let mut visited = vec![vec![false; self.landscape[0].len()]; self.landscape.len()];
        let mut to_visit = vec![self.start_pos];
        for _ in 0..n_steps-2 {
            let mut found_positions = Vec::new();
            while to_visit.len() != 0 {
                let pos = to_visit.pop().expect("to have a position in to_visit");
                //visited[pos.0][pos.1] = true;
                let valid_positions = self.check_surrounding(pos);
                valid_positions.into_iter()
                    .filter(|&p| visited[p.0][p.1].ne(&true))
                    .for_each(|p| found_positions.push(p));
            }
            to_visit.append(&mut found_positions);
            to_visit.iter()
                .for_each(|p| { visited[p.0][p.1] = true; });
        }
        visited.into_iter()
            .map(|v| v.into_iter().filter(|b| b.eq(&true)).count() as u16)
            .sum::<u16>() + 1
    }
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day21.in.txt").expect("file to exist");
    let mut lm: LandscapeMap = input.parse().expect("to be valid map");
    println!("[2023] D21P01: {}", lm.traverse(64));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test() {
        let mut m: LandscapeMap = SAMPLE.parse().expect("to be valid map");
        assert_eq!(16, m.traverse(6));
    }
}
