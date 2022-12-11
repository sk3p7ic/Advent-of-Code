/// Parses a given string into a vector of the lines of that string (the commands).
pub fn parse_into_commands(input: &str) -> Vec<&str> {
    input
        .lines()
        .collect::<Vec<&str>>()
}

/// The program memory / CPU for the device.
pub struct Memory {
    pub cycles: i16,
    pub x_over_time: Vec<i16>
}

impl Memory {
    pub fn init<'a>() -> Memory {
        Memory {
            cycles: 0,
            x_over_time: vec![1i16]
        }
    }

    fn noop(&mut self) {
        self.cycles += 1;
        self.x_over_time.push(*self.x_over_time.last().unwrap());
    }

    fn addx(&mut self, v: i16) {
        self.cycles += 2;
        let last = self.x_over_time.last().unwrap();
        self.x_over_time.extend([*last, *last + v]);
    }
}

/// Gets the desired target cycle signal strengths and returns their sum.
pub fn get_target_cycles(cycles: &Vec<i16>) -> i16 {
    let target_values: [i16; 6] = [
        *cycles.get(19).expect("Failed to get cycle 20 which should exist") * 20,
        *cycles.get(59).expect("Failed to get cycle 60 which should exist") * 60,
        *cycles.get(99).expect("Failed to get cycle 100 which should exist") * 100,
        *cycles.get(139).expect("Failed to get cycle 140 which should exist") * 140,
        *cycles.get(179).expect("Failed to get cycle 180 which should exist") * 180,
        *cycles.get(219).expect("Failed to get cycle 220 which should exist") * 220
    ];
    target_values.into_iter().sum()
}

pub fn process_p1(mem: &mut Memory, commands: Vec<&str>) -> i16 {
    commands
        .into_iter()
        .for_each(|cmd| {
            if cmd.starts_with("addx") {
                let val = cmd.strip_prefix("addx ").unwrap().parse::<i16>().unwrap();
                mem.addx(val);
            }
            else {
                mem.noop();
            }
        });
    get_target_cycles(&mem.x_over_time)
}

/// Renders a character to the current line
fn render_char<'a>(value: &i16, line: &mut Vec<char>) -> String {
    let sprite: [i16; 3] = [value - 1, *value, value + 1];
    let len = line.len() as i16;
    if sprite.contains(&len) {
        line.push('#');
    }
    else {
        line.push('.')
    }
    line.iter().collect::<String>()
}

pub fn process_p2(commands: Vec<&str>) -> String {
    let mut crt: Vec<String> = Vec::new();
    let mut current_line: Vec<char> = Vec::new();
    let mut x_reg = 1i16;
    commands
        .into_iter()
        .for_each(|cmd| {
            if cmd.starts_with("noop") {
                current_line = render_char(&x_reg, &mut current_line).chars().collect();
                // If this line has met the target length
                if current_line.len() == 40 {
                    let full_line = current_line .iter().collect::<String>();
                    crt.push(full_line);
                    current_line.clear();
                }
            }
            else {
                current_line = render_char(&x_reg, &mut current_line).chars().collect();
                if current_line.len() == 40 {
                    let full_line = current_line .iter().collect::<String>();
                    crt.push(full_line);
                    current_line.clear();
                }
                current_line = render_char(&x_reg, &mut current_line).chars().collect();
                if current_line.len() == 40 {
                    let full_line = current_line .iter().collect::<String>();
                    crt.push(full_line);
                    current_line.clear();
                }
                x_reg += cmd.strip_prefix("addx ").unwrap().parse::<i16>().unwrap();
            }
        });
    let full_line = current_line .iter().collect::<String>();
    crt.push(full_line);
    current_line.clear();
    crt.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_p1() {
        let commands: Vec<&str> = parse_into_commands(&INPUT);
        let mem: &mut Memory = &mut Memory::init();
        let result = process_p1(mem, commands);
        assert_eq!(result, 13140);
    }

    #[test]
    fn test_p2() {
        let commands: Vec<&str> = parse_into_commands(&INPUT);
        let result = process_p2(commands);
        dbg!(result);
    }
}
