use day1::process_p1;
use day1::process_p2;
use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    println!("D1P1: {}", process_p1(&input).to_string());
    println!("D1P2: {}", process_p2(&input).to_string());
}
