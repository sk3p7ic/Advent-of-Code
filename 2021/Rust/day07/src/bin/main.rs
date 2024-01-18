use day07::{prepare_input, process_p1, process_p2};

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("file to exist");
    let values = prepare_input(&input);
    println!("D07P1: {}", process_p1(&values));
    println!("D07P1: {}", process_p2(&values));
}
