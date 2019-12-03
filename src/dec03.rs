use crate::read_input;
use std::io::BufRead;
use std::cmp::{min, max};
use std::i64;
use std::usize;
use std::ops::Add;
use std::iter::FromIterator;

fn in_range(from: i64, to: i64, val: i64) -> bool {
    min(from, to) <= val && max(from, to) >= val
}

#[derive(Copy, Clone, PartialEq)]
enum Direction { Up, Down, Left, Right }

impl Direction {
    fn vertical(self) -> bool {
        use Direction::{Up, Down};
        self == Up || self == Down
    }
}

impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        use Direction::*;
        match c {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
struct Path {
    dir: Direction,
    steps: usize,
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let dir = Direction::from(&s[..1]);
        let steps = s[1..].parse::<usize>().expect("could not parse steps");
        Self { dir, steps }
    }
}

#[derive(Default, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self { Self { x, y } }

    fn steps_to(&self, pt: Self) -> i64 { ((self.x - pt.x).abs() + (self.y - pt.y).abs()) }
}

impl Add<Path> for Point {
    type Output = Point;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Path) -> Self::Output {
        use Direction::*;

        let d = rhs.steps as i64;

        match rhs.dir {
            Up => Self { x: self.x, y: self.y + d },
            Down => Self { x: self.x, y: self.y - d },
            Right => Self { x: self.x + d, y: self.y },
            Left => Self { x: self.x - d, y: self.y },
        }
    }
}

#[derive(Copy, Clone)]
struct Segment {
    start: Point,
    end: Point,
    path: Path,
}

impl Segment {
    fn new(start: Point, path: Path) -> Self {
        let end = start + path;
        Self { start, end, path }
    }

    fn intersects(&self, other: &Self) -> Option<Point> {
        let self_vertical = self.path.dir.vertical();
        let other_vertical = other.path.dir.vertical();

        if self_vertical == other_vertical {
            return None;
        }

        if other_vertical && !self_vertical {
            return other.intersects(self);
        }

        let intersects = in_range(self.start.y, self.end.y, other.start.y)
            && in_range(other.start.x, other.end.x, self.start.x);

        if intersects {
            Some(Point::new(self.start.x, other.start.y))
        } else {
            None
        }
    }

    fn contains(&self, pt: Point) -> bool {
        in_range(self.start.x, self.end.x, pt.x)
            && in_range(self.start.y, self.end.y, pt.y)
    }
}

struct Wire(Vec<Segment>);

impl Wire {
    fn intersections(&self, other: &Self) -> Vec<Point> {
        other.into_iter().flat_map(|other_segment| {
            self.into_iter()
                .map(move |segment| segment.intersects(other_segment))
        })
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect()
    }

    fn steps_to(&self, pt: Point) -> usize {
        let mut out = 0;

        for segment in &self.0 {
            if segment.contains(pt) {
                out += segment.start.steps_to(pt) as usize;
                break;
            }
            out += segment.path.steps;
        }

        out
    }

    fn min_distance(&self, other: &Self) -> i64 {
        self.intersections(other).into_iter().fold(i64::MAX, |out, int| {
            match (int.x, int.y) {
                (0, 0) => out,
                (x, y) => min(out, x.abs() + y.abs()),
            }
        })
    }

    fn min_steps(&self, other: &Self) -> usize {
        self.intersections(other).into_iter().fold(usize::MAX, |out, int| {
            match (int.x, int.y) {
                (0, 0) => out,
                _ => min(out, self.steps_to(int) + other.steps_to(int)),
            }
        })
    }
}

impl From<Vec<Segment>> for Wire {
    fn from(v: Vec<Segment>) -> Self { Self(v) }
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let mut last: Point = Point::default();
        Self::from_iter(s.split(',')
            .map(Path::from)
            .map(|path| {
                let seg = Segment::new(last, path);
                last = seg.end;
                seg
            }))
    }
}

impl From<String> for Wire {
    fn from(s: String) -> Self { Self::from(s.as_ref()) }
}

impl FromIterator<Segment> for Wire {
    fn from_iter<T: IntoIterator<Item=Segment>>(iter: T) -> Self {
        <Vec<Segment> as FromIterator<Segment>>::from_iter(iter).into()
    }
}

impl IntoIterator for Wire {
    type Item = <Vec<Segment> as IntoIterator>::Item;
    type IntoIter = <Vec<Segment> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl<'a> IntoIterator for &'a Wire {
    type Item = <&'a Vec<Segment> as IntoIterator>::Item;
    type IntoIter = <&'a Vec<Segment> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter { (&self.0).iter() }
}

fn input_to_wires(name: &str) -> Vec<Wire> {
    read_input(name).lines()
        .map(|l| l.expect("could not read line"))
        .map(Wire::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_dist() {
        let calc_min_dist = |a: &str, b: &str| {
            let first = Wire::from(a);
            let second = Wire::from(b);
            first.min_distance(&second)
        };

        assert_eq!(calc_min_dist(
            "R8,U5,L5,D3",
            "U7,R6,D4,L4",
        ), 6);

        assert_eq!(calc_min_dist(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ), 159);

        assert_eq!(calc_min_dist(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ), 135);
    }

    #[test]
    fn test_min_steps() {
        let calc_min_steps = |a: &str, b: &str| {
            let first = Wire::from(a);
            let second = Wire::from(b);
            first.min_steps(&second)
        };

        assert_eq!(calc_min_steps(
            "R8,U5,L5,D3",
            "U7,R6,D4,L4",
        ), 30);

        assert_eq!(calc_min_steps(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        ), 610);

        assert_eq!(calc_min_steps(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        ), 410);
    }

    #[test]
    fn parts_1_and_2() {
        let wires = input_to_wires("dec03.txt");
        assert_eq!(wires.len(), 2);
        assert_eq!(wires[0].min_distance(&wires[1]), 4981);
        assert_eq!(wires[0].min_steps(&wires[1]), 164_012);
    }
}