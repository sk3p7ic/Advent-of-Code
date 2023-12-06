#[derive(Debug)]
struct Input {
    times: Vec<u16>,
    dists: Vec<u16>
}

impl std::str::FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().split_once("\n").expect("to be valid input");
        let times = lines.0.split_once(":").expect("to have label").1
            .split_ascii_whitespace()
            .map(|n| n.parse::<u16>().expect("to be valid time number"))
            .collect();
        let dists = lines.1.split_once(":").expect("to have label").1
            .split_ascii_whitespace()
            .map(|n| n.parse::<u16>().expect("to be valid dist number"))
            .collect();
        Ok(Self { times, dists })
    }
}

fn gen_btn_presses(times: &Vec<u16>) -> Vec<Vec<u16>> {
    times.iter()
        .map(|&t| (1..=t).map(|hld| hld * (t-hld)).collect::<Vec<u16>>())
        .collect()
}

fn process_p1(input: &str) -> u32 {
    let sheet = input.parse::<Input>().expect("to be valid input");
    let dists_in_times = gen_btn_presses(&sheet.times);
    sheet.dists.iter().enumerate()
        .map(|(i, sd)| dists_in_times[i].iter()
            .filter(|&cd| cd > sd)
            .count() as u32)
        .product()
}

fn process_p2(input: &str) -> usize {
    let sheet = input.parse::<Input>().expect("to be valid input");
    let time = sheet.times.into_iter()
        .map(|t| format!("{}", t))
        .collect::<String>()
        .parse::<u64>().expect("to be a valid number");
    let dist = sheet.dists.into_iter()
        .map(|d| format!("{}", d))
        .collect::<String>()
        .parse::<u64>().expect("to be a valid number");
    let holds = (1..=time).map(|hld| hld * (time - hld)).collect::<Vec<_>>();
    holds.into_iter()
        .filter(|&d| d > dist)
        .count()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day06.in.txt").expect("file to exist");
    println!("[2023] D06P01: {}", process_p1(&input));
    println!("[2023] D06P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn p1_works() {
        assert_eq!(288, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(71503, process_p2(&SAMPLE));
    }
}
