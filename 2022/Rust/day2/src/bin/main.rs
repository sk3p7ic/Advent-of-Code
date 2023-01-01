use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let score_1 = day2::run_p1(&input);
    let score_2 = day2::run_p2(&input);
    println!("D2P1: {}", score_1);
    println!("D2P2: {}", score_2);
}
