use day08::process_p1;

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("file to exist");
    println!("D08P1: {}", process_p1(&input));
}
