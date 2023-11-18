pub struct P1Result {
    pub power: u32,
    pub mcb: u16,
    pub lcb: u16
}

pub fn process_p1(input: &str) -> P1Result {
    let lines: Vec<String> = to_line_vec(input);
    let mut most_common_str: String = String::new();
    for i in 0..lines[0].len() {
        let mut zero_one_counts = (0, 0);
        for bs in &lines {
            match bs.chars().nth(i).expect("character to exist") {
                '0' => zero_one_counts = (zero_one_counts.0 + 1, zero_one_counts.1),
                '1' => zero_one_counts = (zero_one_counts.0, zero_one_counts.1 + 1),
                n => panic!("Recieved '{}' when 0 or 1 expected.", n)
            }
        }
        if zero_one_counts.0 > zero_one_counts.1 {
            most_common_str += "0";
        } else {
            most_common_str += "1";
        }
    }
    let mut lcb_mask = String::new();
    for i in 0..16 {
        if i < 16 - most_common_str.len() {
            lcb_mask += "0";
        } else {
            lcb_mask += "1";
        }
    }
    let lcb_mask = u16::from_str_radix(&lcb_mask, 2).expect("string to be bits");
    let mcb = u16::from_str_radix(&most_common_str, 2).expect("string to be bits");
    let mut lcb = !mcb;
    lcb &= lcb_mask;
    return P1Result{power: (mcb as u32) * (lcb as u32), mcb, lcb};
}

pub fn process_p2(input: &str) -> u32 {
    let mut mcb_lines = to_line_vec(&input);
    let mut lcb_lines = to_line_vec(&input);
    let mut passes = mcb_lines.get(0).expect("value to exist").len();
    while mcb_lines.len() != 1 {
        for i in 0..passes {
            let mut vline: String = String::new();
            for line in &mcb_lines {
                vline += line.as_str();
                vline += "\n";
            }
            let res = process_p1(vline.as_str());
            let mcb = strip_start(format!("{:016b}", res.mcb).as_str(), 16-mcb_lines.get(0).unwrap().len());
            let bit = mcb.chars().nth(i).expect("char to exist");
            mcb_lines = mcb_lines.into_iter().filter(|s| s.chars().nth(i).unwrap() == bit).collect();
            if mcb_lines.len() == 1 {
                break;
            }
        }
    }
    passes = lcb_lines.get(0).expect("value to exist").len();
    while lcb_lines.len() != 1 {
        for i in 0..passes {
            let mut vline: String = String::new();
            for line in &lcb_lines {
                vline += line.as_str();
                vline += "\n";
            }
            let res = process_p1(vline.as_str());
            let lcb = strip_start(format!("{:016b}", res.lcb).as_str(), 16-lcb_lines.get(0).unwrap().len());
            let bit = lcb.chars().nth(i).expect("char to exist");
            lcb_lines = lcb_lines.into_iter().filter(|s| s.chars().nth(i).unwrap() == bit).collect();
            if lcb_lines.len() == 1 {
                break;
            }
        }
    }
    let mcb = u32::from_str_radix(mcb_lines.get(0).expect("value to exist in vec"),
        2).expect("string to be bits");
    let lcb = u32::from_str_radix(lcb_lines.get(0).expect("value to exist in vec"),
        2).expect("string to be bits");
    return mcb * lcb;
}

pub fn old_process_p2(input: &str, mcb: u16, lcb: u16) -> u32 {
    let mut mcb_lines = to_line_vec(&input);
    let mut lcb_lines = to_line_vec(&input);
    let mut mcb_str = format!("{:016b}", mcb);
    let mut lcb_str = format!("{:016b}", lcb);
    mcb_str = strip_start(&mcb_str, 16 - mcb_lines[0].len());
    lcb_str = strip_start(&lcb_str, 16 - mcb_lines[0].len());
    for i in 0..mcb_str.len() {
        let mut removable_indices: Vec<usize> = Vec::new();
        let mut offset = 0;
        for j in 0..mcb_lines.len() {
            let bs = &mcb_lines.get(j).unwrap();
            if removable_indices.len() < mcb_lines.len() - 1 &&
                bs.chars().nth(i).unwrap() != mcb_str.chars().nth(i).unwrap() {
                removable_indices.push(j - offset);
                offset += 1;
            }
        }
        for idx in removable_indices {
            mcb_lines.remove(idx);
        }
    }
    for i in 0..lcb_str.len() {
        let mut removable_indices: Vec<usize> = Vec::new();
        let mut offset = 0;
        for j in 0..lcb_lines.len() {
            let bs = &lcb_lines.get(j).unwrap();
            if removable_indices.len() < lcb_lines.len() - 1 &&
                bs.chars().nth(i).unwrap() != lcb_str.chars().nth(i).unwrap() {
                removable_indices.push(j - offset);
                offset += 1;
            }
        }
        for idx in removable_indices {
            lcb_lines.remove(idx);
        }
    }
    let mcs = mcb_lines.get(0).expect("vec to have single value");
    let lcs = lcb_lines.get(0).expect("vec to have single value");
    let mcb = u32::from_str_radix(mcs, 2).expect("string to be bits");
    let lcb = u32::from_str_radix(lcs, 2).expect("string to be bits");
    return mcb * lcb;
}

fn to_line_vec(input: &str) -> Vec<String> {
    input.trim().lines()
        .map(|s| s.to_string())
        .collect()
}

fn strip_start(input: &str, n: usize) -> String {
    let mut res = String::new();
    for i in n..input.len() {
        res.push(input.chars().nth(i).expect("character to exist"));
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn p1_works() {
        let res = process_p1(&SAMPLE);
        assert_eq!(res.power, 198);
    }

    #[test]
    fn p2_works() {
        assert_eq!(process_p2(&SAMPLE), 230);
    }
}
