use std::collections::VecDeque;

pub fn to_vd(input: &str) -> VecDeque<i16> {
    input
        .split("\n")
        .map(|n| n.parse::<i16>().unwrap())
        .collect::<VecDeque<i16>>()
}

pub fn mix(mut buf: VecDeque<i16>) -> VecDeque<i16> {
    let orig = buf.clone();
    for idx in 0..buf.len()-1 {
        let mut shifts = orig.get(idx).unwrap() % buf.len() as i16;
        let is_neg = shifts < 0;
        if is_neg {
            shifts *= -1;
        }
        for p in 1..shifts as usize {
            dbg!(&buf);
            if is_neg {
                buf.swap(idx, idx-p);
            }
            else {
                buf.swap(idx, idx+p);
            }
        }
    }
    return buf
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_input() {
        let input = to_vd(INPUT);
        let output = mix(input);
    }
}
