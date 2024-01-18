use day02::{process_p1, process_p2};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    println!("D2P1: {}", process_p1(input.as_str()));
    println!("D2P2: {}", process_p2(input.as_str()));
}
