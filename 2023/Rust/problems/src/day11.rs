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

fn matrix_to_string(mtx: &Vec<Vec<TileType>>) -> String {
    let mut s = String::new();
    for v in mtx {
        s.push_str(v.iter().map(|t| match t {
                TileType::Empty => ".",
                TileType::Galaxy => "#",
            }.to_string())
            .collect::<String>().as_str());
        s.push('\n');
    }
    s.trim().to_string()
}

fn parse_and_expand(input: &str) -> Vec<Vec<TileType>> {
    let mut given: Vec<Vec<TileType>> = input.trim().lines()
        .map(|line| line.chars().map(|c| c.to_string().as_str().parse().expect("to be of a valid TileType"))
            .collect())
        .collect();
    // Perform expansions
    let mut to_expand: Vec<usize> = Vec::new();
    for (i, r) in given.iter().enumerate() {
        if r.iter().all(|t| match t { TileType::Empty => true, _ => false }) {
            to_expand.push(i);
        }
    }
    for (i, e) in to_expand.into_iter().enumerate() {
        let mut nr = Vec::new();
        for _ in 0..given[0].len() { nr.push(TileType::Empty); }
        given.insert(e + i, nr);
    }
    to_expand = Vec::new();
    for col in 0..given[0].len() {
        if given.iter().all(|r| match r[col] { TileType::Empty => true, _ => false }) {
            to_expand.push(col);
        }
    }
    for (i, e) in to_expand.into_iter().enumerate() {
        for r in 0..given.len() {
            given[r].insert(e+i, TileType::Empty);
        }
    }
    given
}

fn process_p1(mtx: &Vec<Vec<TileType>>) -> usize {
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for r in 0..mtx.len() {
        for c in 0..mtx[0].len() {
            if mtx[r][c].eq(&TileType::Galaxy) {
                coords.push((r, c));
            }
        }
    }
    // Inefficient O(n^2) solu
    let mut dists: Vec<usize> = Vec::new();
    for (idx, base) in coords.clone().into_iter().enumerate() {
        for (jdx, coord) in coords.clone().into_iter().enumerate() {
            if jdx <= idx { continue; }
            let dist = coord.0.abs_diff(base.0) + coord.1.abs_diff(base.1);
            dists.push(dist);
        }
    }
    dists.into_iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day11.in.txt").expect("file to exist");
    let expanded = parse_and_expand(&input);
    println!("[2023] D11P01: {}", process_p1(&expanded));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PLAIN: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const SAMPLE_EXPANDED: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

    #[test]
    fn expansion_works() {
        let expanded = parse_and_expand(&SAMPLE_PLAIN);
        assert_eq!(SAMPLE_EXPANDED, matrix_to_string(&expanded).as_str());
    }

    #[test]
    fn p1_works() {
        let expanded = parse_and_expand(&SAMPLE_PLAIN);
        assert_eq!(374, process_p1(&expanded));
    }
}
