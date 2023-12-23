use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign};
use std::str::FromStr;

pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

pub fn p<T>(s: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    s.parse().unwrap()
}

pub fn ints_g<T, C: FromIterator<T>>(s: &str) -> C
where
    T: FromStr,
    T::Err: Debug,
{
    let signed = "-1".parse::<T>().is_ok();

    s.split(|c: char| !(c.is_numeric() || (signed && c == '-')))
        .filter(|s| !s.is_empty())
        .map(p)
        .collect()
}

pub fn ints<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    ints_g(s)
}

pub fn ints_n<T, const N: usize>(s: &str) -> [T; N]
where
    T: FromStr,
    T::Err: Debug,
{
    ints(s).try_into().ok().unwrap()
}

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos<N> {
    pub x: N,
    pub y: N,
}

impl<N: Debug> Debug for Pos<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<N> Pos<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn pair(self) -> (N, N) {
        (self.x, self.y)
    }
}

impl<N: Add<Output = N>> Add<(N, N)> for Pos<N> {
    type Output = Self;
    fn add(self, (x, y): (N, N)) -> Self::Output {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl<N: AddAssign<N>> AddAssign<(N, N)> for Pos<N> {
    fn add_assign(&mut self, (x, y): (N, N)) {
        self.x += x;
        self.y += y;
    }
}

impl<N> From<(N, N)> for Pos<N> {
    fn from((x, y): (N, N)) -> Self {
        Self { x, y }
    }
}

impl<N> From<Pos<N>> for (N, N) {
    fn from(p: Pos<N>) -> Self {
        (p.x, p.y)
    }
}

pub fn parse_input_lines<T, F: FnMut(&'static str) -> T>(input_data: &'static str, f: F) -> Vec<T> {
    input_data.lines().map(str::trim).map(f).collect()
}

pub fn run<Parser, Part1, Part2, In, Out>(
    input_data: &'static str,
    parser: Parser,
    part1: Part1,
    part2: Part2,
) where
    Parser: FnMut(&'static str) -> In,
    Part1: FnOnce(&[In]) -> Out,
    Part2: FnOnce(&[In]) -> Out,
    Out: Display,
{
    let input = parse_input_lines(input_data, parser);

    let output1 = part1(&input);
    println!("{output1}");

    let output2 = part2(&input);
    println!("{output2}");
}

pub fn run_alt<Parser, Part1, Part2, In, Out>(
    input_data: &'static str,
    parser: Parser,
    part1: Part1,
    part2: Part2,
) where
    Parser: FnOnce(&'static str) -> In,
    Part1: FnOnce(&In) -> Out,
    Part2: FnOnce(&In) -> Out,
    Out: Display,
{
    let input = parser(input_data);

    let output1 = part1(&input);
    println!("{output1}");

    let output2 = part2(&input);
    println!("{output2}");
}

pub fn test<Parser, Part, In, Out>(
    test_data: &'static str,
    output_data: &'static str,
    parser: Parser,
    part: Part,
    part2: bool,
) where
    Parser: FnMut(&'static str) -> In,
    Part: FnOnce(&[In]) -> Out,
    Out: Debug + FromStr + PartialEq,
    Out::Err: Debug,
{
    let input = parse_input_lines(test_data, parser);
    let xy = parse_output::<Out>(output_data);
    let x = if part2 { xy.1 } else { xy.0 };

    assert_eq!(part(&input), x);
}

pub fn test_alt<Parser, Part, In, Out>(
    test_data: &'static str,
    output_data: &'static str,
    parser: Parser,
    part: Part,
    part2: bool,
) where
    Parser: FnOnce(&'static str) -> In,
    Part: FnOnce(&In) -> Out,
    Out: Debug + FromStr + PartialEq,
    Out::Err: Debug,
{
    let input = parser(test_data);
    let xy = parse_output::<Out>(output_data);
    let x = if part2 { xy.1 } else { xy.0 };

    assert_eq!(part(&input), x);
}

pub fn parse_output<T>(output_data: &'static str) -> (T, T)
where
    T: FromStr,
    T::Err: Debug,
{
    let (a, b) = output_data.split_once("\n").unwrap();
    let [x, y] = [a, b].map(p::<T>);
    (x, y)
}

#[macro_export]
macro_rules! register {
    ($parser:expr, $part1:expr, $part2:expr) => {
        $crate::register!($parser, $part1, $part2, run, test);
    };

    ($parser:expr, $part1:expr, $part2:expr, @alt) => {
        $crate::register!($parser, $part1, $part2, run_alt, test_alt);
    };

    ($parser:expr, $part1:expr, $part2:expr, $run:ident, $test:ident) => {
        const INPUT: &str = include_str!("../input.txt");

        fn main() {
            $crate::$run(INPUT, $parser, $part1, $part2);
        }

        #[cfg(test)]
        extern crate test;

        const TEST_INPUT: &str = include_str!("../test.txt");
        const TEST_OUTPUT: &str = include_str!("../test.out.txt");

        #[cfg(test)]
        #[test]
        fn test_part1() {
            $crate::$test(TEST_INPUT, TEST_OUTPUT, $parser, $part1, false);
        }

        #[cfg(test)]
        #[test]
        fn test_part2() {
            $crate::$test(TEST_INPUT, TEST_OUTPUT, $parser, $part2, true);
        }
    };
}

// TODO: benchmarking
