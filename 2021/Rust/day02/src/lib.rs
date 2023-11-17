pub fn process_p1(input: &str) -> u32 {
    let mut coords: (u16, u16) = (0, 0);
    let commands = input.trim().lines();
    for c in commands {
        let mut c_iter = c.split(" ");
        let (direction, amount) = (c_iter.next().unwrap(),
            c_iter.next().unwrap().parse::<u16>().unwrap());
        match direction {
            "forward" => {
                coords = (coords.0 + amount, coords.1);
            },
            "down" => {
                coords = (coords.0, coords.1 + amount);
            },
            "up" => {
                coords = (coords.0, coords.1 - amount);
            },
            _ => {}
        }
    }
    (coords.0 as u32) * (coords.1 as u32)
}

pub fn process_p2(input: &str) -> u32 {
    let mut coords: (u32, u32) = (0, 0);
    let mut aim = 0;
    let commands = input.trim().lines();
    for c in commands {
        let mut c_iter = c.split(" ");
        let (direction, amount) = (c_iter.next().unwrap(),
            c_iter.next().unwrap().parse::<u32>().unwrap());
        match direction {
            "forward" => {
                coords = (coords.0 + amount, coords.1 + amount * aim);
            },
            "down" => {
                aim += amount;
            },
            "up" => {
                aim -= amount;
            },
            _ => {}
        }
    }
    coords.0 * coords.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn p1_works() {
        let res = process_p1(SAMPLE);
        assert_eq!(res, 150);
    }

    #[test]
    fn p2_works() {
        let res = process_p2(SAMPLE);
        assert_eq!(res, 900);
    }
}
