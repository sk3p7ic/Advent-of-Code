use std::collections::LinkedList;

#[derive(Clone)]
struct Lens {
    label: String,
    focal: usize
}

struct Lenses {
    boxes: Vec<LinkedList<Lens>>
}

impl Lenses {
    fn new() -> Self {
        let boxes = (0..256).map(|_| LinkedList::new()).collect();
        Self { boxes }
    }

    fn update(&mut self, s: &str) {
        let (label, focal) = s.split_once(&['-', '=']).expect("to be splittable with '='");
        let box_idx = label.chars()
            .map(|c| c as usize)
            .fold(0, |acc, n| ((acc + n) * 17) % 256);
        if s.contains("=") {
            let lens = Lens {
                label: label.to_string(),
                focal: focal.parse().expect("to be valid focal length")
            };
            let mut b = self.boxes[box_idx].clone();
            let mut did_modify = false;
            for elem in b.iter_mut() {
                if elem.label == label.to_string() {
                    elem.focal = lens.focal;
                    did_modify = true;
                }
            }
            if did_modify {
                self.boxes[box_idx] = b;
            } else {
                self.boxes[box_idx].push_back(lens);
            }
        }
        else if s.contains("-") {
            let mut b = self.boxes[box_idx].clone();
            for (idx, elem) in b.iter_mut().enumerate() {
                if elem.label == label.to_string() {
                    let mut rest = self.boxes[box_idx].split_off(idx + 1);
                    self.boxes[box_idx].pop_back();
                    self.boxes[box_idx].append(&mut rest);
                }
            }
        }
    }

    fn powers(&self) -> usize {
        self.boxes.iter().enumerate()
            .map(|(i, v)| v.iter().enumerate()
                .map(|(j, lens)| (i + 1) * (j + 1) * lens.focal)
                .sum::<usize>())
            .sum()
    }
}

fn process_p1(input: &str) -> u32 {
    input.trim().split(",")
        .map(|s| s.chars()
            .map(|c| c as u32)
            .fold(0, |acc, n| ((acc + n) * 17) % 256))
        .sum()
}

fn process_p2(input: &str) -> usize {
    let mut lenses = Lenses::new();
    input.trim().split(",")
        .for_each(|s| lenses.update(s));
    lenses.powers()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day15.in.txt").expect("file to exist");
    println!("[2023] D15P01: {}", process_p1(&input));
    println!("[2023] D15P02: {}", process_p2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn p1_works() {
        assert_eq!(1320, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(145, process_p2(&SAMPLE));
    }
}
