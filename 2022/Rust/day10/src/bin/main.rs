use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let p1_commands = day10::parse_into_commands(&input);
    let mem = &mut day10:: Memory::init();
    let p1_result = day10::process_p1(mem, p1_commands);
    let p2_commands = day10::parse_into_commands(&input);
    let p2_result = day10::process_p2(p2_commands);
    println!("D10P1: {}", p1_result);
    println!("D10P2:\n{}", p2_result);
}
