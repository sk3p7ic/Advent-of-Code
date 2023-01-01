pub fn process_p1(input: &str) -> u32 {
    return input
        .split("\n\n")
        .map(|cals| {
            cals
                .split("\n")
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        }).max().unwrap();
}

pub fn process_p2(input: &str) -> u32 {
    let mut cal_vec = input
        .split("\n\n")
        .map(|cals| {
            cals
                .split("\n")
                .map(|c| c.parse::<u32>().unwrap())
                .sum::<u32>()
        }).collect::<Vec<_>>();
    cal_vec.sort_by(|a, b| b.cmp(a));
    return cal_vec.iter().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_one() {
        let p1 = process_p1(INPUT);
        assert_eq!(p1, 24000);
    }

    #[test]
    fn test_two() {
        let p2 = process_p2(INPUT);
        assert_eq!(p2, 45000)
    }
}
