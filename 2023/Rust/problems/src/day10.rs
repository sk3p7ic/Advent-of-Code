#[derive(Clone, Copy, Debug)]
enum TileType {
    Start, Ground, NorthSouth, EastWest, NorthEast, NorthWest, SouthWest,
    SouthEast
}

impl std::str::FromStr for TileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::NorthSouth),
            "-" => Ok(Self::EastWest),
            "L" => Ok(Self::NorthEast),
            "J" => Ok(Self::NorthWest),
            "7" => Ok(Self::SouthWest),
            "F" => Ok(Self::SouthEast),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err(format!("Unknown direction token '{}'.", s).to_string())
        }
    }
}

#[derive(Debug)]
struct Tile {
    coord: (usize, usize),
    tile: TileType
}

fn input_to_matrix(input: &str) -> Vec<Vec<TileType>> {
    input.trim().lines()
        .map(|line| line.chars()
            .map(|c| c.to_string().parse().expect("to be valid Tile"))
            .collect())
        .collect()
}

fn cartesian_matrix(mtx: &Vec<Vec<TileType>>) -> Vec<Vec<Tile>> {
    let mut new_mtx = Vec::new();
    for r in 0..mtx.len() {
        let mut new_row = Vec::new();
        for c in 0..mtx[0].len() {
            new_row.push(Tile { coord: (r, c), tile: mtx[r][c] });
        }
        new_mtx.push(new_row);
    }
    new_mtx
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day10.in.txt").expect("file to exist");
    println!("Input: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn p1_works() {
        let mtx = cartesian_matrix(&input_to_matrix(&SAMPLE));
        dbg!(mtx);
        assert_eq!(1, 0);
    }
}
