pub fn process_p1(input: &str) -> usize {
    let starting_fish = parse_input(input);
    let fish_counts_after_days = get_all_fish_counts(80);
    let n_each_fish = vec![0, 1, 2, 3, 4, 5, 6, 7, 8].iter()
        .map(|&f| starting_fish.iter().filter(|&fv| *fv == f).count())
        .collect::<Vec<usize>>();
    n_each_fish.iter()
        .enumerate()
        .map(|(i, n)| n * fish_counts_after_days.get(i).expect("count to exist"))
        .sum()
}

pub fn process_p2(input: &str) -> usize {
    let starting_fish = parse_input(input);
    // Thanks to https://github.com/duarten for aiding me in this issue
    // and letting me know that my original idea was overthinking things
    let mut counts: [usize; 9] = [0; 9];
    starting_fish.iter()
        .for_each(|f| counts[*f] += 1);
    for _ in 0..256 {
        for (i, f) in std::mem::take(&mut counts).into_iter().enumerate() {
            if i == 0 {
                counts[6] += f;
                counts[8] += f;
            } else {
                counts[i-1] += f;
            }
        }
    }
    counts.into_iter().sum()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.trim().split(",")
        .map(|f| f.parse::<usize>().expect("to be integer value"))
        .collect()
}

fn simulate_fish_from_day(starting_day: i8, n_days: usize) -> usize {
    let mut fish_vec: Vec<i8> = Vec::new();
    fish_vec.push(starting_day);
    for _ in 0..n_days {
        for i in 0..fish_vec.len() {
            let mut fish = fish_vec.get(i).expect("value to exist");
            if *fish - 1 == -1 {
                fish = &7;
                fish_vec.push(8);
            }
            fish_vec[i] = *fish - 1;
        }
    }
    fish_vec.len()
}

fn get_all_fish_counts(n_days: usize) -> [usize; 9] {
    let mut all_counts = [0; 9];
    for d in 0..all_counts.len() {
        all_counts[d] = simulate_fish_from_day(d as i8, n_days);
    }
    all_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn test_fish_gen() {
        assert_eq!(simulate_fish_from_day(6, 15), 3);
    }

    #[test]
    fn p1_works() {
        assert_eq!(process_p1(&SAMPLE), 5934);
    }

    #[test]
    fn p2_works() {
        assert_eq!(process_p2(&SAMPLE), 26984457539);
    }
}
