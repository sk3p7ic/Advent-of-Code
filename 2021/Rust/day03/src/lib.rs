pub fn process_p1(input: &str) -> u32 {
    let lines: Vec<String> = to_line_vec(input);
    let mut most_common_str: String = String::new();
    for i in 0..lines[0].len() {
        most_common_str += most_common_at_index(&lines, i).to_string().as_str();
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
    return (mcb as u32) * (lcb as u32);
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
            let bit = most_common_at_index(&to_line_vec(vline.as_str()), i);
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
            let bit = match most_common_at_index(&to_line_vec(vline.as_str()), i) {
                '0' => '1',
                '1' => '0',
                n => panic!("Recieved '{}' when 0 or 1 expected.", n)
            };
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

fn to_line_vec(input: &str) -> Vec<String> {
    input.trim().lines()
        .map(|s| s.to_string())
        .collect()
}

fn most_common_at_index(lines: &Vec<String>, index: usize) -> char {
    let mut zero_one_counts = (0, 0);
    for bs in lines {
        match bs.chars().nth(index).expect("character to exist") {
            '0' => zero_one_counts = (zero_one_counts.0 + 1, zero_one_counts.1),
            '1' => zero_one_counts = (zero_one_counts.0, zero_one_counts.1 + 1),
            n => panic!("Recieved '{}' when 0 or 1 expected.", n)
        }
    }
    if zero_one_counts.0 > zero_one_counts.1 {
        return '0';
    } else {
        return '1';
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn p1_works() {
        let res = process_p1(&SAMPLE);
        assert_eq!(res, 198);
    }

    #[test]
    fn p2_works() {
        assert_eq!(process_p2(&SAMPLE), 230);
    }
}
