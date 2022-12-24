use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let total_1 = day3::do_p1(&input);
    let total_2 = day3::do_p2(&input);
    println!("Day 03, Part 01: {}", total_1);
    println!("Day 03, Part 02: {}", total_2);
}
