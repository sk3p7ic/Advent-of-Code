struct Bag {
    r: u16,
    g: u16,
    b: u16
}


fn prepare(input: &str) -> Vec<Vec<String>> {
    input.trim().lines()
        .map(|line| line.split_once(':')
            .expect("line to be of valid syntax")
            .1
            .trim()
            .replace(",", "")
            .replace(";", "")
            .split(" ")
            .map(|g| g.to_string())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}


fn process_p1(input: &str) -> u16 {
    const BAG: Bag = Bag{r: 12, g: 13, b: 14};
    let mut permissible_games: Vec<u16> = Vec::new();
    let games = prepare(input);
    for (i, g) in games.into_iter().enumerate() {
        let mut allowed = true;
        let mut curr_num = 0u16;
        for v in g.iter() {
            if let Ok(cn) = v.parse::<u16>() {
                curr_num = cn;
                continue;
            }
            if !match v.as_str() {
                "red" => curr_num <= BAG.r,
                "green" => curr_num <= BAG.g,
                "blue" => curr_num <= BAG.b,
                c => panic!("Recieved some unknown '{c}'")
            } {
                allowed = false;
                break;
            }
        }
        if allowed {
            permissible_games.push((i as u16) + 1);
        }
    }
    permissible_games.into_iter().sum()
}


fn process_p2(input: &str) -> u32 {
    let mut powers: Vec<u32> = Vec::new();
    let games = prepare(input);
    for g in games.into_iter() {
        let mut bag: Bag = Bag{r: 0, g: 0, b: 0};
        let mut curr_num = 0u16;
        for v in g.iter() {
            if let Ok(cn) = v.parse::<u16>() {
                curr_num = cn;
                continue;
            }
            match v.as_str() {
                "red" => bag.r = std::cmp::max(curr_num, bag.r),
                "green" => bag.g = std::cmp::max(curr_num, bag.g),
                "blue" => bag.b = std::cmp::max(curr_num, bag.b),
                c => panic!("Recieved some unknown '{c}'")
            }
        }
        powers.push((bag.r * bag.g * bag.b) as u32);
    }
    powers.into_iter().sum()
}


fn main() {
    let input = std::fs::read_to_string("./inputs/day02.in.txt").expect("file to exist");
    println!("[2023] D02P01: {}", process_p1(&input));
    println!("[2023] D02P02: {}", process_p2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn p1_works() {
        assert_eq!(8, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(2286, process_p2(&SAMPLE));
    }
}
