use day03::{process_p1, process_p2};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("file to exist");
    println!("D3P1: {}", process_p1(&input).power);
    println!("D3P1: {}", process_p2(&input));
}
