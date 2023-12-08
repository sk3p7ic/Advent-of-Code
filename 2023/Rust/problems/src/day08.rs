use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Command { Left, Right }

impl std::str::FromStr for Command {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(format!("Invalid token, '{}'...", s))
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    this: String,
    left: String,
    right: String,
}

impl std::str::FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_once(" = ").expect("to be a valid line");
        let this = parts.0.to_string();
        let (left, right) = parts.1.trim_matches(|c| c == '(' || c == ')')
            .split_once(", ").expect("to have both children");
        Ok(Self { this, left: left.to_string(), right: right.to_string() })
    }
}

#[derive(Debug)]
struct Instructions {
    commands: Vec<Command>,
    nodes: HashMap<String, Node>
}

impl std::str::FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        let commands = s.trim().lines()
            .next().expect("line with commands to exist").chars()
            .map(|c| c.to_string().as_str().parse().expect("to be valid command"))
            .collect::<Vec<Command>>();
        s.trim().lines()
            .skip(2)
            .map(|line| line.parse::<Node>().expect("to be a valid Node"))
            .for_each(|n| {
                let nc = n.clone();
                nodes.insert(nc.this.clone(), nc); });
        Ok(Self { commands, nodes })
    }
}

impl Instructions {
    fn find_starts(&self) -> Vec<String> {
        self.nodes.keys().clone().into_iter()
            .map(|s| (s, s.chars().last().expect("to have a last character")))
            .filter(|(_, lst)| lst == &'A')
            .map(|t| t.0.clone())
            .collect()
    }
}

fn traverse(instrs: &Instructions) -> usize {
    let mut curr = instrs.nodes.get(&"AAA".to_string()).expect("first value to exist");
    let mut dir_ptr = 0;
    while !curr.this.eq_ignore_ascii_case("ZZZ") {
        let dir = instrs.commands[dir_ptr % instrs.commands.len()];
        match dir {
            Command::Left => {
                let nxt = instrs.nodes.get(&curr.left).expect("left node to exist");
                curr = nxt;
            },
            Command::Right => {
                let nxt = instrs.nodes.get(&curr.right).expect("right node to exist");
                curr = nxt;
            }
        }
        dir_ptr += 1;
    }
    dir_ptr
}

fn multi_traverse(instrs: &Instructions) -> usize {
    let start_labels = instrs.find_starts();
    let mut currs = start_labels.into_iter()
        .map(|lbl| instrs.nodes.get(&lbl).expect("node to exist"))
        .collect::<Vec<_>>();
    let mut dir_ptr = 0;
    while !currs.iter()
        .all(|&c| c.this.clone().chars()
            .last().expect("to have last character")
            .eq_ignore_ascii_case(&'Z')) {
        let dir = instrs.commands[dir_ptr % instrs.commands.len()];
        match dir {
            Command::Left => {
                let nexts = currs.iter()
                    .map(|&c| instrs.nodes.get(&c.left).expect("left node to exist"))
                    .collect::<Vec<_>>();
                currs = nexts;
            },
            Command::Right => {
                let nexts = currs.iter()
                    .map(|&c| instrs.nodes.get(&c.right).expect("right node to exist"))
                    .collect::<Vec<_>>();
                currs = nexts;
            }
        }
        dir_ptr += 1;
    }
    dir_ptr
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day08.in.txt").expect("file to exist");
    let instructions = input.parse().expect("to be valid instructions");
    println!("[2023] D08P01: {}", traverse(&instructions));
    println!("[2023] D08P02: {}", multi_traverse(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_2: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_6: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_P2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn p1_works_for_2() {
        let instrs = SAMPLE_2.parse::<Instructions>().expect("to be valid instructions");
        dbg!(&instrs);
        assert_eq!(2, traverse(&instrs));
    }

    #[test]
    fn p1_works_for_6() {
        let instrs = SAMPLE_6.parse::<Instructions>().expect("to be valid instructions");
        dbg!(&instrs);
        assert_eq!(6, traverse(&instrs));
    }

    #[test]
    fn p2_works() {
        let instrs = SAMPLE_P2.parse::<Instructions>().expect("to be valid instructions");
        dbg!(&instrs);
        assert_eq!(6, multi_traverse(&instrs));
    }
}
