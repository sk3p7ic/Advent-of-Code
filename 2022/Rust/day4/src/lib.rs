use std::collections::HashSet;

pub fn do_p1(input: &str) -> u16 {
    input
        .lines()
        .map(|line|
            line
                .split(",")
                .map(|side| side.split("-").map(|n| n.parse::<u16>().unwrap()).collect::<Vec<u16>>())
                .collect::<Vec<Vec<u16>>>()
        )
        .map(|vals| {
            if (vals[0][0] <= vals[1][0] && vals[0][1] >= vals[1][1]) || (vals[0][0] >= vals[1][0] && vals[0][1] <= vals[1][1]) {
                return 1
            } else {
                return 0
            }
        })
        .sum()
}

pub fn do_p2(input: &str) -> u16 {
    input
        .lines()
        .map(|line|
            line
                .split(",")
                .map(|side| side.split("-").map(|n| n.parse::<u16>().unwrap()).collect::<Vec<u16>>())
                .collect::<Vec<Vec<u16>>>()
        )
        .map(|vals| {
            let pair1: HashSet<u16> = HashSet::from_iter(vals[0][0]..vals[0][1] + 1);
            let pair2: HashSet<u16> = HashSet::from_iter(vals[1][0]..vals[1][1] + 1);
            for v in pair1 {
                if pair2.contains(&v) {
                    return 1
                }
            }
            return 0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_p1() {
        let p1 = do_p1(&INPUT);
        assert_eq!(p1, 2u16);
    }

    #[test]
    fn test_p2() {
        let p2 = do_p2(&INPUT);
        assert_eq!(p2, 4u16);
    }
}
