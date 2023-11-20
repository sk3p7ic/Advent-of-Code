use day04::{process_p1, process_p2};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("file to exist");
    println!("D4P1: {}", process_p1(&input));
    println!("D4P2: {}", process_p2(&input));
}
