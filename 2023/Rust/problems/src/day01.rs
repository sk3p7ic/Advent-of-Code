fn process_p1(input: &str) -> u16 {
    input.trim().lines()
        .map(|line| (
            line.chars()
                .filter(|c| c.is_digit(10))
                .next().expect("digit to exist")
                .to_digit(10).expect("to be convertible") as u16,
            line.chars()
                .filter(|c| c.is_digit(10))
                .last().expect("digit to exist")
                .to_digit(10).expect("to be convertible") as u16
            )
        )
        .map(|(t, o)| t * 10 + o)
        .sum()
}

fn process_p2(input: &str) -> u16 {
    let mut new_input: String = input.to_string();
    const DIGITS: [&str; 10] = [ "zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"];
    for (i, d) in DIGITS.into_iter().enumerate() {
        let first = d.chars().next().expect("first character to exist");
        let last = d.chars().last().expect("last character to exist");
        new_input = new_input.replace(d, format!("{first}{i}{last}").as_str());
    }
    process_p1(&new_input)
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day01.in.txt").expect("file to exist");
    println!("[2023] D01P01: {}", process_p1(&input));
    println!("[2023] D01P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_works() {
        const SAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(process_p1(&SAMPLE), 142);
    }

    #[test]
    fn p2_works() {
        const SAMPLE: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(process_p2(&SAMPLE), 281);
    }
}
