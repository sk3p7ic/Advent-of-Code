use std::collections::HashMap;

pub fn process_p1(input: &str) -> usize {
    input.trim().lines()
        .map(|line| line.split_once(" | ").expect("to be valid line").1.split(" ")
            .map(|output| match output.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0
            })
            .filter(|&ln| ln == 1)
            .count())
        .sum()
}

pub fn process_p2(input: &str) -> usize {
    let sorted = input.trim().lines()
        .map(|line| line.split_once(" | ").expect("to be valid line"))
        .map(|parts| sort_line(parts))
        .collect::<Vec<(String, String)>>();
    let mut maps: Vec<HashMap<String, u8>> = vec![];
    for (sig_group, _) in sorted.iter() {
        let mut hm: HashMap<_, _> = HashMap::default();
        for sig in sig_group.split(" ") {
            hm.insert(sig.to_string(),
                match sig.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    _ => 0
                }
            );
        }
        maps.push(hm);
    }
    0
}

fn sort_line(line: (&str, &str)) -> (String, String) {
    let mut left_char_vecs = line.0.split(" ")
        .map(|seg| seg.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut right_char_vecs = line.1.split(" ")
        .map(|seg| seg.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for (idx, mut cv) in left_char_vecs.clone().into_iter().enumerate() {
        cv.sort_unstable();
        left_char_vecs[idx] = cv;
    }
    for (idx, mut cv) in right_char_vecs.clone().into_iter().enumerate() {
        cv.sort_unstable();
        right_char_vecs[idx] = cv;
    }
    let mut left_strs = left_char_vecs.into_iter()
        .map(|cv| cv.into_iter().collect())
        .collect::<Vec<String>>();
    left_strs.sort_unstable();
    let mut right_strs = right_char_vecs.into_iter()
        .map(|cv| cv.into_iter().collect())
        .collect::<Vec<String>>();
    right_strs.sort_unstable();
    (left_strs.join(" "), right_strs.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn p1_works() {
        assert_eq!(process_p1(&SAMPLE), 26);
    }
}
