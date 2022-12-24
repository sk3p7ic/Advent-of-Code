use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let p1 = day4::do_p1(&input);
    let p2 = day4::do_p2(&input);
    println!("Day 04, Part 01: {}", p1);
    println!("Day 04, Part 02: {}", p2);
}
