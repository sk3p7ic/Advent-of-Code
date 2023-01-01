pub fn convert_to_ints(input: &str) -> Vec<Vec<u8>> {
    let parsed: _ = input
        .split("\n")
        .map(|r: &str| {
            r
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .map(|t| t as u8)
                .map(|t| t - 48u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
parsed
}

pub fn from_vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!(
            "Sizes do not match! Expected {} but got {}",
            N,
            v.len()
    ))
}

pub fn process_p1<const N: usize>(forest: &[[u8; N]; N]) -> u32 {
    let mut num_visible = 0u32;

    for y in 0..N {
        for x in 0..N {
            let tree = &forest[y][x];

            // Check left
            let left = &forest[y][0..x];
            if left.len() == 0 {
                num_visible += 1;
                continue;
            }
            else {
                if *left.iter().max().unwrap() < *tree {
                    num_visible += 1;
                    continue;
                }
            }

            // Check right
            let right = &forest[y][x + 1..N];
            if right.len() == 0 {
                num_visible += 1;
                continue;
            }
            else {
                if *right.iter().max().unwrap() < *tree {
                    num_visible += 1;
                    continue;
                }
            }

            // Check up
            if y == 0 {
                num_visible += 1;
                continue;
            }
            let up_max = &forest
                .into_iter()
                .map(|r| r[x])
                .max()
                .unwrap();
            if *up_max < *tree {
                num_visible += 1;
                continue;
            }

            // Check down
            if y == N - 1 {
                num_visible += 1;
                continue;
            }
            let down_max = &forest
                .into_iter()
                .map(|r| r[x])
                .max()
                .unwrap();
            if *down_max < *tree {
                num_visible += 1;
                continue;
            }
        }
    }
    num_visible
}


pub fn process_p2<const N: usize>(forest: &[[u8; N]; N]) -> f64 {
    let mut max_score: f64 = 0f64;

    for y in 0..N {
        for x in 0..N {
            let tree = &forest[y][x];

            // Check left
            let mut score_left: u16 = 0;
            let left = &forest[y][..x];
            for dx in 0..left.len() {
                score_left += 1;
                if left[left.len() - 1 - dx] >= *tree {
                    break;
                }
            }

            // Check right
            let mut score_right: u16 = 0;
            let right = &forest[y][x + 1..N];
            for dx in 0..right.len() {
                score_right += 1;
                if right[dx] >= *tree {
                    break;
                }
            }

            // Check up
            let mut score_up: u16 = 0;
            let up = &forest[0..y]
                .into_iter()
                .map(|r| r[x])
                .collect::<Vec<u8>>();
            for dy in 0..up.len() {
                score_up += 1;
                if *up.get(up.len() - 1 - dy).unwrap() >= *tree {
                    break;
                }
            }

            // Check down
            let mut score_down: u16 = 0;
            let down = &forest[y + 1..N]
                .into_iter()
                .map(|r| r[x])
                .collect::<Vec<u8>>();
            for dy in 0..down.len() {
                score_down += 1;
                if *down.get(dy).unwrap() >= *tree {
                    break;
                }
            }

            // Get the total score for this tree
            let score_this = (score_left * score_right * score_up * score_down) as f64;
            if score_this > max_score {
                max_score = score_this;
            }
        }
    }

    max_score
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";
    const DIMEN: usize = 5;

    #[test]
    fn test_part1() {
        let parsed = convert_to_ints(INPUT);
        let converted_inner: _ = parsed
            .into_iter()
            .map(|r| from_vec_to_arr(r))
            .collect::<Vec<[u8; DIMEN]>>();
        let forest: [[u8; DIMEN]; DIMEN] = from_vec_to_arr(converted_inner);
        let num_visible = process_p1(&forest);
        assert_eq!(num_visible, 21);
    }

    #[test]
    fn test_part2() {
        let parsed = convert_to_ints(INPUT);
        let converted_inner: _ = parsed
            .into_iter()
            .map(|r| from_vec_to_arr(r))
            .collect::<Vec<[u8; DIMEN]>>();
        let forest: [[u8; DIMEN]; DIMEN] = from_vec_to_arr(converted_inner);
        let max_score = process_p2(&forest);
        assert_eq!(max_score, 8f64);
    }
}
