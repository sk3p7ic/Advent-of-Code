use day01::{to_vec, process_p1, process_p2};
use std::fs;

fn main() {
    let input = to_vec(fs::read_to_string("./input.txt").unwrap().as_str());
    println!("D1P1: {}", process_p1(input.as_slice()));
    println!("D1P2: {}", process_p2(input.as_slice()));
}
