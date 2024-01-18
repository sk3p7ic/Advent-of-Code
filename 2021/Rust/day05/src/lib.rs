use std::{iter::repeat, collections::HashMap};

#[derive(Eq, PartialEq, Hash)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl std::str::FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').expect("to be a point");
        Ok(Self {
            x: x.parse::<u16>().expect("to be a valid x coordinate"),
            y: y.parse::<u16>().expect("to be a valid y coordinate")
        })
    }
}


/// Indicates whether a point is on a diagonal or non-diagonal line.
///
/// For the purposes of this problem, a diagonal line is defined as any
/// drawable line whereas a non-diagonal line is defined as a line that is
/// either horizontal or vertical.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum LineType { Diag, NonDiag }

/// Represents a line between two points.
pub struct Line {
    pub p1: Point,
    pub p2: Point,
}

impl std::str::FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p1, p2) = s.split_once(" -> ").expect("to be a line");
        Ok(Line { p1: p1.parse()?, p2: p2.parse()? })
    }
}

impl Line {
    /// Returns a vector of integer points that are on the line, inclusive of
    /// the start and end points. Each point is paired with a `LineType` that
    /// indicates whether the point is on a diagonal or non-diagonal line.
    /// Refer to `LineType` for more information.
    pub fn points(&self) -> Vec<(LineType, Point)> {
        match self.is_diag() {
            true => Line::gen_points(
                Line::gen_range(self.p1.x, self.p2.x),
                Line::gen_range(self.p1.y, self.p2.y),
                LineType::Diag
            ),
            false => {
                if self.p1.x == self.p2.x {
                    Line::gen_points(repeat(self.p1.x), Line::gen_range(self.p1.y, self.p2.y), LineType::NonDiag)
                } else if self.p1.y == self.p2.y {
                    Line::gen_points(Line::gen_range(self.p1.x, self.p2.x), repeat(self.p1.y), LineType::NonDiag)
                } else {
                    vec![]
                }
            }
        }
    }

    /// Returns whether the line is diagonal.
    fn is_diag(&self) -> bool {
        return self.p1.x.abs_diff(self.p2.x) == self.p1.y.abs_diff(self.p2.y)
    }

    /// Generates a range of integers from two integers.
    ///
    /// # Arguments
    /// * `a` - The first integer.
    /// * `b` - The second integer.
    ///
    /// # Returns
    /// An iterator of integers from `a` to `b` if `a <= b`, otherwise an
    /// iterator of integers from `b` to `a` in reverse order, both inclusive.
    fn gen_range(a: u16, b: u16) -> Box<dyn Iterator<Item = u16>> {
        match a <= b {
            true => Box::new(a..=b),
            false => Box::new((b..=a).rev())
        }
    }

    /// Generates a vector of points from two ranges of integers.
    ///
    /// # Arguments
    /// * `xs` - An iterator of x coordinates.
    /// * `ys` - An iterator of y coordinates.
    /// * `t` - The type of line that the points are on. Refer to `LineType`
    ///   for more information.
    fn gen_points<RngItr1, RngItr2>(xs: RngItr1, ys: RngItr2, t: LineType) -> Vec<(LineType, Point)>
    where
        RngItr1: Iterator<Item = u16>,
        RngItr2: Iterator<Item = u16>
    {
        xs.zip(ys).map(|(x, y)| (t, Point { x, y })).collect()
    }
}

/// Returns the number of intersections between lines.
///
/// # Arguments
/// * `lines` - A slice of lines.
/// * `filter` - The type of lines to count intersections for. If
///   `LineType::Diag`, then all lines are counted. If `LineType::NonDiag`,
///   then only non-diagonal lines are counted.
pub fn intersections(lines: &[Line], filter: LineType) -> usize {
    let mut map: HashMap<Point, usize> = HashMap::default();
    for p in lines.iter().flat_map(Line::points) {
        match filter {
            LineType::Diag => *map.entry(p.1).or_insert(0) += 1,
            LineType::NonDiag => {
                if p.0 == LineType::NonDiag {
                    *map.entry(p.1).or_insert(0) += 1;
                }
            }
        }
    }
    map.values().filter(|&&v| v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn p1_works() {
        let lines: Vec<Line> = SAMPLE.lines()
            .map(|ln| ln.parse::<Line>().expect("to be valid line"))
            .collect();
        assert_eq!(intersections(&lines, LineType::NonDiag), 5);
    }

    #[test]
    fn p2_works() {
        let lines: Vec<Line> = SAMPLE.lines()
            .map(|ln| ln.parse::<Line>().expect("to be valid line"))
            .collect();
        assert_eq!(intersections(&lines, LineType::Diag), 12);
    }
}
