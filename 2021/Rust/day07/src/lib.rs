pub fn process_p1(vals: &[u32]) -> u32 {
    gen_iter(&vals).into_iter()
        .map(|offset| {
            vals.iter()
                .map(|&v| v.abs_diff(offset as u32))
                .sum::<u32>()
        })
        .min()
        .expect("minimum to exist") as u32
}

pub fn process_p2(vals: &[u32]) -> u32 {
    gen_iter(&vals).into_iter()
        .map(|offset| {
            vals.iter()
                .map(|&v| (1..=v.abs_diff(offset as u32)).into_iter().sum::<u32>())
                .sum::<u32>()
        })
        .min()
        .expect("minimum to exist") as u32
}

fn gen_iter(vals: &[u32]) -> impl Iterator<Item = usize> {
    let avg = vals.iter().sum::<u32>() as usize / vals.len();
    let dev = vals.len() / 4; // Not really a deviation, but w/e
    avg-dev..=avg+dev
}

pub fn prepare_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(",")
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn p1_works() {
        let values = prepare_input(&SAMPLE);
        assert_eq!(process_p1(&values), 37);
    }

    #[test]
    fn p2_works() {
        let values = prepare_input(&SAMPLE);
        assert_eq!(process_p2(&values), 168);
    }
}
