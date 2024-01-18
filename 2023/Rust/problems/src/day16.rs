#[derive(Clone, Copy, Debug)]
enum VisitState { Unvisited, VisitTop, VisitRight, VisitBottom, VisitLeft }

#[derive(Debug)]
struct Contraption {
    grid: Vec<Vec<char>>,
    visits: Vec<Vec<[VisitState; 2]>>
}

impl std::str::FromStr for Contraption {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.trim().lines()
            .map(|line| line.chars().collect())
            .collect();
        let visits = grid.iter()
            .map(|v| v.iter().map(|_| [VisitState::Unvisited; 2]).collect())
            .collect();
        Ok(Self { grid, visits })
    }
}

impl Contraption {
    fn get_new_pos(&self, pos: (usize, usize), trans_tup: (isize, isize)) -> Option<(usize, usize)> {
        let npos = ((pos.0 as isize) + trans_tup.0, (pos.1 as isize) + trans_tup.1);
        if npos.0 < 0 || npos.0 >= (self.grid[0].len() as isize) ||
            npos.1 < 0 || (npos.1 >= self.grid.len() as isize) {
            return None;
        }
        Some((npos.0 as usize, npos.1 as usize))
    }

    fn is_valid_pos(&self, pos: (usize, usize)) -> bool {
        //match self.get_new_pos(pos, trans_tup) {
        //    Some(_) => true,
        //    None => false
        //}
        pos.0 < self.grid[0].len() && pos.1 < self.grid.len()
    }

    fn visit_guard(&self, pos: (usize, usize), trans_tup: (isize, isize)) -> bool {
        if let Some(p) = self.get_new_pos(pos, trans_tup) {
            match self.visits[p.0][p.1] {
                [_, VisitState::Unvisited] => true,
                _ => false
            }
        }
        else { return false; }
    }

    fn update_visit_state(&mut self, pos: (usize, usize), trans_tup: (isize, isize)) {
        match self.visits[pos.0][pos.1] {
            [VisitState::Unvisited, VisitState::Unvisited] => {
                self.visits[pos.0][pos.1][0] = match trans_tup {
                    (-1, 0) => VisitState::VisitRight,
                    (1, 0) => VisitState::VisitLeft,
                    (0, -1) => VisitState::VisitTop,
                    (0, 1) => VisitState::VisitBottom,
                    (_, _) => unreachable!()
                }
            },
            [_, VisitState::Unvisited] => {
                self.visits[pos.0][pos.1][1] = match trans_tup {
                    (-1, 0) => VisitState::VisitRight,
                    (1, 0) => VisitState::VisitLeft,
                    (0, -1) => VisitState::VisitTop,
                    (0, 1) => VisitState::VisitBottom,
                    (_, _) => unreachable!()
                }
            },
            [_, _] => unreachable!() // I think this is unreachable.
        }
    }

    fn traverse(&mut self, mut pos: (usize, usize), mut trans_tup: (isize, isize)) {
        self.update_visit_state(pos, trans_tup);
        while self.is_valid_pos(pos) && self.visit_guard(pos, trans_tup) {
            pos = self.get_new_pos(pos, trans_tup).expect("to be valid position");
            self.update_visit_state(pos, trans_tup);
            match (self.grid[pos.0][pos.1], trans_tup) {
                ('/', (_, _)) => {
                    trans_tup = (trans_tup.1, trans_tup.0);
                },
                ('\\', (_, _)) => {
                    trans_tup = (trans_tup.1 * -1, trans_tup.0 * -1);
                },
                ('-', (_, -1)) | ('-', (_, 1)) => {
                    trans_tup = (trans_tup.1, 0);
                    self.traverse(pos.clone(), (trans_tup.0 * -1, 0));
                },
                ('|', (-1, _)) | ('|', (1, _)) => {
                    trans_tup = (0, trans_tup.0);
                    self.traverse(pos.clone(), (0, trans_tup.1 * -1));
                },
                ('-', (_, 0)) | ('|', (0, _)) | (_, (_, _)) => {}
            }
        }
    }

    fn count_energized(&self) -> usize {
        self.visits.iter()
            .map(|v| v.iter()
                .filter(|state| match state {
                    [VisitState::Unvisited, VisitState::Unvisited] => false,
                    _ => true})
                .count())
            .sum()
    }

    fn show_visits(&self) {
        let s = self.visits.iter()
            .map(|v| v.iter()
                .map(|state| match state {
                    [VisitState::Unvisited, VisitState::Unvisited] => '.',
                    _ => '#' })
                .collect::<String>())
            .map(|s| format!("{}\n", s))
            .collect::<String>();
        println!("{}", s);
    }
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day16.in.txt").expect("file to exist");
    println!("Input: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn p1_works() {
        let mut contr: Contraption = SAMPLE.parse().unwrap();
        contr.traverse((0, 0), (1, 0));
        contr.show_visits();
        assert_eq!(46, contr.count_energized());
    }
}
