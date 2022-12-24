use std::collections::HashSet;

fn to_priority(c: &char) -> u32 {
    if c.is_lowercase() {
        return (*c as u32) - 96;
    } else {
        return (*c as u32) - 38;
    }
}

fn process_line(input: &str) -> u32 {
    let split_index: usize = input.len() / 2;
    let (lhs, rhs) = input.split_at(split_index);
    let mut char_set: HashSet<char> = HashSet::new();
    for c in lhs.chars() {
        char_set.insert(c);
    }
    for c in rhs.chars() {
        if char_set.contains(&c) {
            return to_priority(&c);
        }
    }
    return 0;
}

pub fn do_p1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| process_line(line))
        .sum()
}

fn process_group(line_1: &str, line_2: &str, line_3: &str) -> u32 {
    let mut char_set_1: HashSet<char> = HashSet::new();
    let mut char_set_2: HashSet<char> = HashSet::new();
    for c in line_1.chars() {
        char_set_1.insert(c);
    }
    for c in line_2.chars() {
        char_set_2.insert(c);
    }
    for c in line_3.chars() {
        if char_set_1.contains(&c) && char_set_2.contains(&c) {
            return to_priority(&c);
        }
    }
    return 0;
}

pub fn do_p2(input: &str) -> u32 {
    let lines = input
        .lines()
        .collect::<Vec<&str>>();
    let mut sum = 0u32;
    for group in lines.chunks(3) {
        sum += process_group(group[0], group[1], group[2]);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_p1() {
        let res = do_p1(&INPUT);
        assert_eq!(res, 157);
    }

    #[test]
    fn test_p2() {
        let res = do_p2(&INPUT);
        assert_eq!(res, 70);
    }
}
