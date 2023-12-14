fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    (0..v[0].len()).map(|i|
        v.iter().map(|iv| iv[i].clone()).collect())
        .collect()
}

fn parse(s: &str) -> Vec<Vec<char>> {
    s.trim().lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn rotate_dish(dish: &mut Vec<Vec<char>>) {
    for (rnum, row) in dish.clone().iter().enumerate() {
        let mut new_row = row.clone();
        let round_rocks = row.iter().enumerate()
            .filter_map(|(i, r)| match r { 'O' => Some(i), _ => None })
            .collect::<Vec<usize>>();
        round_rocks.into_iter()
            .for_each(|ri| {
                let mut pos = ri;
                while pos > 0 && !match new_row[pos-1] { '#' | 'O' => true, _ => false } {
                    new_row[pos-1] = 'O';
                    new_row[pos] = '.';
                    pos -= 1;
                }
            });
        dish[rnum] = new_row;
    }
}

fn process(input: &str, n_rotations: usize) -> usize {
    let mut dish = parse(&input);
    for _ in 0..n_rotations {
        dish = transpose(dish);
        rotate_dish(&mut dish);
    }
    dish.into_iter()
        .map(|v| v.iter().enumerate()
            .filter(|(_, &r)| r == 'O')
            .map(|(i, _)| v.len() - i)
            .sum::<usize>()).sum()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day14.in.txt").expect("file to exist");
    println!("[2023] D14P01: {}", process(&input, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn p1_works() {
        assert_eq!(136, process(&SAMPLE, 1));
    }

    #[test]
    fn p2_works() {
        assert_eq!(64, process(&SAMPLE, 4));
    }
}
