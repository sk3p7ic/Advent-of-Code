use day05::{Line, LineType, intersections};

pub fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("file to exist");
    let lines: Vec<Line> = input.trim().lines()
        .map(|ln| ln.parse().expect("to be a valid line"))
        .collect();
    println!("D05P1: {}", intersections(&lines, LineType::NonDiag));
    println!("D05P2: {}", intersections(&lines, LineType::Diag));
}
