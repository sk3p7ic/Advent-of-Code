use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    const DIMEN: usize = 99;
    let mut parsed = day8::convert_to_ints(&input);
    parsed.pop(); // Remove last element (it's empty)
    let converted_inner: _ = parsed
        .into_iter()
        .map(|r| day8::from_vec_to_arr(r))
        .collect::<Vec<[u8; DIMEN]>>();
    let forest: [[u8; DIMEN]; DIMEN] = day8::from_vec_to_arr(converted_inner);
    let num_visible = day8::process_p1(&forest);
    //let max_score = day8::process_p2(&forest);

    println!("D8P1: {}", num_visible);
    //println!("D8P2: {}", max_score);
}
