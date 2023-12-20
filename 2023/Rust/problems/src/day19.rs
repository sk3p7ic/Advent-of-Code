use std::collections::HashMap;

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}

impl std::str::FromStr for Rating {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self{ x: 0, m: 0, a: 0, s: 0};
        let mut err: Option<String> = None;
        s.trim_matches(|c| c == '{' || c == '}').split(",")
            .map(|assignment| assignment.split_once("=").expect("to be valid assignment"))
            .for_each(|(v, n)| {match v {
                "x" => this.x = n.parse().expect("to be valid x number"),
                "m" => this.m = n.parse().expect("to be valid m number"),
                "a" => this.a = n.parse().expect("to be valid a number"),
                "s" => this.s = n.parse().expect("to be valid s number"),
                _ => { err = Some(format!("Attempted to assign '{}' to token '{}'.", n, v)); }
            }});
        if let Some(e) = err {
            return Err(e);
        }
        Ok(this)
    }
}

#[derive(Debug)]
enum Resolution { Accept, Reject, Forward(String) }

#[derive(Debug)]
enum Comparison { LessThan((String, usize)), GreaterThan((String, usize)) }

#[derive(Debug)]
enum WorkflowItem {
    Comparable((Comparison, Resolution)),
    Resolvable(Resolution),
}

#[derive(Debug)]
struct Workflow {
    actions: Vec<WorkflowItem>
}

impl std::str::FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<String> = s.trim_matches(|c| c == '{' || c == '}').split(",")
            .map(|x| x.to_string())
            .collect();
        let mut actions = Vec::new();
        for opr in ops {
            if opr.contains(":") {
                let (left, right) = opr.split_once(":").expect("to have colon");
                let resolution = match right {
                    "A" => Resolution::Accept,
                    "R" => Resolution::Reject,
                    val => Resolution::Forward(val.to_string())
                };
                let (v, n) = left.split_at(1);
                let is_lt = match n.chars().next().expect("char to exist") {
                    '<' => true,
                    '>' => false,
                    c => panic!("{} -- {} ({})", opr, c, n)
                };
                let n = n.trim_matches(|c| c == '<' || c == '>').parse().expect("to be valid number");
                let comp = match is_lt {
                    true => Comparison::LessThan((v.to_string(), n)),
                    false => Comparison::GreaterThan((v.to_string(), n))
                };
                actions.push(WorkflowItem::Comparable((comp, resolution)));
            } else {
                let this_op = opr.clone();
                actions.push(match this_op.as_str() {
                    "A" => WorkflowItem::Resolvable(Resolution::Accept),
                    "R" => WorkflowItem::Resolvable(Resolution::Reject),
                    val => WorkflowItem::Resolvable(Resolution::Forward(val.to_string()))
                });
            }
        }
        Ok(Self{ actions })
    }
}

#[derive(Debug)]
struct PartsSystem {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>
}

impl std::str::FromStr for PartsSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (w, r) = s.trim().split_once("\n\n").expect("to have double newline");
        let mut workflows = HashMap::new();
        w.lines()
            .map(|line| line.split_once("{").expect("to be valid workflow line"))
            .map(|(left, right)| (left, Workflow::from_str(right).expect("to be valid workflow")))
            .for_each(|(left, right)| {
                workflows.insert(left.to_string(), right);
            });
        let ratings = r.split("\n")
            .map(|r_str| r_str.parse().expect("to be valid rating"))
            .collect();
        Ok(Self{ workflows, ratings })
    }
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day19.in.txt").expect("file to exist");
    println!("Input: {}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn p1_works() {
        let parts: PartsSystem = SAMPLE.parse().expect("to be valid system");
        dbg!(parts);
        assert_eq!(1, 0);
    }
}
