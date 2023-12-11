use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TileType { Empty, Galaxy }

impl std::str::FromStr for TileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Galaxy),
            _ => Err(format!("Bad token '{}'...", s))
        }
    }
}

#[derive(Debug)]
struct BlankLineReport {
    rows: HashSet<usize>,
    cols: HashSet<usize>,
}

fn find_blank_lines(mtx: &Vec<Vec<TileType>>) -> BlankLineReport {
    let mut rows = HashSet::new();
    for (i, r) in mtx.iter().enumerate() {
        if r.iter().all(|t| match t { TileType::Empty => true, _ => false }) {
            rows.insert(i);
        }
    }
    let mut cols = HashSet::new();
    for col in 0..mtx[0].len() {
        if mtx.iter().all(|r| match r[col] { TileType::Empty => true, _ => false }) {
            cols.insert(col);
        }
    }
    BlankLineReport { rows, cols }
}

fn parse(input: &str) -> Vec<Vec<TileType>> {
    input.trim().lines()
        .map(|line| line.chars().map(|c| c.to_string().as_str().parse().expect("to be of a valid TileType"))
            .collect())
        .collect()
}

fn sum_distances(mtx: &Vec<Vec<TileType>>, mut expansion_len: usize) -> usize {
    let blanks = find_blank_lines(&mtx);
    expansion_len -= 1;
    dbg!(&blanks);
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for r in 0..mtx.len() {
        for c in 0..mtx[0].len() {
            if mtx[r][c].eq(&TileType::Galaxy) {
                coords.push((r, c));
            }
        }
    }
    // Inefficient O(n^2) solu using Manhattan distance
    let mut dists: Vec<usize> = Vec::new();
    for (idx, base) in coords.clone().into_iter().enumerate() {
        for (jdx, coord) in coords.clone().into_iter().enumerate() {
            if jdx <= idx { continue; }
            let dist = coord.0.abs_diff(base.0) + coord.1.abs_diff(base.1);
            let x_range = match coord.0 <= base.0 {
                true => coord.0..base.0,
                false => base.0..coord.0
            };
            let y_range = match coord.1 <= base.1 {
                true => coord.1..base.1,
                false => base.1..coord.1
            };
            let x_set: HashSet<usize> = HashSet::from_iter(x_range);
            let y_set: HashSet<usize> = HashSet::from_iter(y_range);
            let dx = blanks.rows.intersection(&x_set).count() * expansion_len;
            let dy = blanks.cols.intersection(&y_set).count() * expansion_len;
            //dbg!(((base, coord), dx, dy));
            dists.push(dist + dx + dy);
        }
    }
    dists.into_iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day11.in.txt").expect("file to exist");
    let parsed = parse(&input);
    println!("[2023] D11P01: {}", sum_distances(&parsed, 2));
    println!("[2023] D11P02: {}", sum_distances(&parsed, 1000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn p1_works() {
        let expanded = parse(&SAMPLE);
        assert_eq!(374, sum_distances(&expanded, 2));
    }

    #[test]
    fn p2_works() {
        let parsed = parse(&SAMPLE);
        assert_eq!(1030, sum_distances(&parsed, 10));
        assert_eq!(8410, sum_distances(&parsed, 100));
    }
}
