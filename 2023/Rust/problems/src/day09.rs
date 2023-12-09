fn parse(input: &str) -> Vec<Vec<i64>> {
    input.trim().lines()
        .map(|line| line.split_ascii_whitespace().map(|c| c.trim().parse().expect("to be a number")).collect())
        .collect()
}

fn diff_to_zero(line: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_lines = Vec::new();
    let mut last_line = line.clone();
    while !last_line.iter().all(|&n| n == 0) {
        all_lines.push(last_line.clone());
        let mut diffs: Vec<i64> = Vec::new();
        for (i, n) in last_line.clone().into_iter().enumerate() {
            if i == 0 { continue; }
            diffs.push(n - last_line[i - 1]);
        }
        last_line = diffs;
    }
    let mut new_last = Vec::new();
    for _ in 0..last_line.clone().len() - 1 {
        new_last.push(0);
    }
    all_lines.push(new_last);
    all_lines
}

fn extrapolate(lines: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let llen = lines.len();
    let mut all_lines = lines.clone();
    all_lines[llen - 1].push(0);
    for i in (0..lines.len() - 1).rev() {
        let this_len = all_lines[i].len();
        let prev_len = all_lines[i + 1].len();
        let new_val = all_lines[i][this_len - 1] + all_lines[i + 1][prev_len - 1];
        all_lines[i].push(new_val);
    }
    all_lines
}

fn extrapolate_front(lines: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let llen = lines.len();
    let mut all_lines = lines.clone();
    for (i, mut v) in all_lines.clone().into_iter().enumerate() {
        v.reverse();
        all_lines[i] = v;
    }
    all_lines[llen - 1].push(0);
    for i in (0..lines.len() - 1).rev() {
        let this_len = all_lines[i].len();
        let prev_len = all_lines[i + 1].len();
        let new_val = all_lines[i][this_len - 1] - all_lines[i + 1][prev_len - 1];
        all_lines[i].push(new_val);
    }
    all_lines
}

fn process_p1(input: &str) -> i64 {
    let mut lasts: Vec<i64> = Vec::new();
    let data = parse(&input);
    for d in data {
        let lines = diff_to_zero(&d);
        let extra = extrapolate(&lines);
        lasts.push(extra.into_iter()
            .next().expect("to have first line").iter()
            .last().expect("to have last value in line").clone());
    }
    lasts.into_iter().sum()
}

fn process_p2(input: &str) -> i64 {
    let mut lasts: Vec<i64> = Vec::new();
    let data = parse(&input);
    for d in data {
        let lines = diff_to_zero(&d);
        let extra = extrapolate_front(&lines);
        lasts.push(extra.into_iter()
            .next().expect("to have first line").iter()
            .last().expect("to have last value in line").clone());
    }
    lasts.into_iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day09.in.txt").expect("file to exist");
    println!("[2023] D09P01: {}", process_p1(&input));
    println!("[2023] D09P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn p1_works() {
        assert_eq!(114, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(2, process_p2(&SAMPLE));
    }
}
