pub fn process_p1(depths: &[u16]) -> u16 {
    let mut n_incr = 0;
    for i in 0..depths.len() {
        if i == 0 {
            continue;
        }
        if depths[i] > depths[i-1] {
            n_incr += 1;
        }
    }
    return n_incr;
}

pub fn process_p2(depths: &[u16]) -> u16 {
    let mut n_incr = 0;
    let sums = depths.windows(3)
        .map(|w| w.iter().sum::<u16>())
        .collect::<Vec<u16>>();
    let mut prev_sum = std::u16::MAX;
    for s in sums {
        if s > prev_sum {
            n_incr += 1;
        }
        prev_sum = s;
    }
    return n_incr;
}

pub fn to_vec(input: &str) -> Vec<u16> {
    input.trim().split("\n")
        .map(|n| n.parse::<u16>().unwrap())
        .collect::<Vec<u16>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn p1_works() {
        let depths = to_vec(&SAMPLE);
        let n_incr = process_p1(depths.as_slice());
        assert_eq!(n_incr, 7);
    }

    #[test]
    fn p2_works() {
        let depths = to_vec(&SAMPLE);
        let n_incr = process_p2(depths.as_slice());
        assert_eq!(n_incr, 5);
    }
}
