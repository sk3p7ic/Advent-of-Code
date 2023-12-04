fn get_cards(input: &str) -> Vec<Vec<u16>> {
    input.trim().lines()
        .map(|line| line.split_once(": ").expect("line to be valid").1)
        .map(|line| line.split_once(" | ").expect("line to have both parts"))
        .map(|line| (
            line.0.split(" ")
                .map(|v| v.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>().parse::<i8>().unwrap_or(-1))
                .collect::<Vec<_>>(),
            line.1.split(" ")
                .map(|v| v.chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>().parse::<i8>().unwrap_or(-1))
                .collect::<Vec<_>>()))
        .map(|(v1, v2)| v1.iter()
            .filter(|n| v2.contains(&n))
            .map(|v| v.clone())
            .collect::<Vec<i8>>())
        .map(|v| v.iter().filter(|&n| *n >= 0).map(|n| *n as u16).collect::<Vec<u16>>())
        .collect()
}

fn process_p1(input: &str) -> u16 {
    get_cards(&input).into_iter()
        .map(|v| (match v.len() {
            0 => 0,
            n => 2u16.pow((n as u32) - 1u32)
        }) as u16)
        .sum()
}

fn process_p2(input: &str) -> usize {
    let n_cards = input.trim().lines().collect::<Vec<_>>().len();
    let mut card_counts: Vec<usize> = vec![0; n_cards];
    let cards = get_cards(&input);
    for (i, c) in cards.iter().enumerate() {
        card_counts[i] += 1;
        for _ in 0..card_counts[i] {
            for j in i+1..=i+c.len() {
                card_counts[j] += 1;
            }
        }
    }
    //dbg!(&cards);
    //dbg!(&card_counts);
    card_counts.into_iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day04.in.txt").expect("file to exist");
    println!("[2023] D04P01: {}", process_p1(&input));
    println!("[2023] D04P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn p1_works() {
        assert_eq!(13, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(30, process_p2(&SAMPLE));
    }
}
