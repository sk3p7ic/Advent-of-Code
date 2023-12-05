use std::cmp::{max, min};

#[derive(Clone, Debug)]
struct Mapping {
    pub src_start: usize,
    pub dst_start: usize,
    pub len_range: usize,
}

impl std::str::FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim().split(" ");
        Ok(Mapping {
            src_start: iter
                .next().expect("source start to exist")
                .parse::<usize>().expect("to be number"),
            dst_start: iter
                .next().expect("destination start to exist")
                .parse::<usize>().expect("to be number"),
            len_range: iter
                .next().expect("length of range to exist")
                .parse::<usize>().expect("to be number"),
        })
        
    }
}

impl Mapping {
    fn src_range(&self) -> std::ops::Range<usize> {
        self.src_start..self.src_start+self.len_range
    }

    fn dst_range(&self) -> std::ops::Range<usize> {
        self.dst_start..self.dst_start+self.len_range
    }

    fn src_contains_n(&self, n: usize) -> bool {
        self.src_range().contains(&n)
    }

    fn src_contains_range(&self, mut other: std::ops::Range<usize>) -> bool {
        let mut src = self.src_range();
        let mx = max(src.next().unwrap(), other.next().unwrap());
        let mn = min(src.last().unwrap_or(mx), other.last().unwrap_or(mx));
        mx <= mn
    }
}

#[derive(Clone, Debug)]
struct MappingGroup {
    pub label: String,
    pub mappings: Vec<Mapping>
}

impl std::str::FromStr for MappingGroup {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut group_iter = s.split("\n");
        let label = group_iter
            .next().expect("label to exist")
            .trim_end_matches(" map:")
            .to_string();
        let mappings = group_iter
            .map(|g| g.parse().expect("to be a valid Mapping object"))
            .collect::<Vec<Mapping>>();
        Ok(Self { label, mappings })
    }
}

impl MappingGroup {
    fn groups_from_input(input: &str) -> Vec<Self> {
        let mut group_iter = input.trim().split("\n\n").skip(1);
        let mut groups: Vec<MappingGroup> = Vec::new();
        while let Some(g) = group_iter.next() {
            groups.push(g.parse().expect("to be a valid MappingGroup"));
        }
        groups
    }

    fn traversal_from_seed(&self, seed: usize) -> Option<Mapping> {
        let mut potential_mappings: Vec<Mapping> = Vec::new();
        self.clone().mappings.into_iter()
            .filter(|m| m.src_contains_n(seed) == true)
            .for_each(|m| potential_mappings.push(m.clone()));
        match potential_mappings.get(0) {
            None => None,
            Some(m) => Some(m.clone())
        }
    }

    fn traversal_from_range(&self, range: std::ops::Range<usize>) -> Mapping {
        let m = self.clone().mappings.iter()
            .find(|m| m.src_contains_range(range.clone()) == true)
            .expect("Mapping to exist for values")
            .clone();
        m
    }
}

fn process_p1(input: &str) -> usize {
    let seeds: Vec<usize> = input.trim().split_once("\n\n")
        .expect("to be properly formatted")
        .0.split_once(": ").expect("to have seeds")
        .1.split(" ")
        .map(|seed| seed.parse::<usize>().expect("to be a valid number"))
        .collect();
    dbg!(&seeds);
    let groups = MappingGroup::groups_from_input(&input);
    let takes_seed = seeds.iter()
        .map(|&s| (s, groups[0].traversal_from_seed(s)))
        .map(|(s, m)| (s, match m {
            None => s..s+1,
            Some(mapping) => mapping.dst_range()
        }))
        .collect::<Vec<_>>();
    dbg!(&takes_seed);
    let mut takes: Vec<(usize, std::ops::Range<usize>)> = takes_seed;
    for i in 1..groups.len()-1 {
        takes = takes.into_iter()
            .map(|t| (t.0, groups[i].traversal_from_range(t.1.clone())))
            .map(|t| (t.0, t.1.dst_range()))
            .collect();
    }
    dbg!(takes);
    //let takes_soil = takes_seed.iter()
    //    .map(|s| (s.0, groups[1].traversal_from_range(s.1.clone())))
    //    .collect::<Vec<_>>();
    //dbg!(&takes_soil);
    0
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day05.in.txt").expect("file to exist");
    println!("[2023] D05P01: {}", process_p1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn p1_works() {
        assert_eq!(35, process_p1(&SAMPLE));
    }
}
