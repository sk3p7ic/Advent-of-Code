fn main() {
    let input = std::fs::read_to_string("./inputs/day02.in.txt").expect("file to exist");
    println!("Input: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
