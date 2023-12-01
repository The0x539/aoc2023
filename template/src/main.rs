#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type In = N;
type Out = usize;

fn parse(s: &'static str) -> In {
    p(s)
}

fn part1(n: &[In]) -> Out {
    Default::default()
}

fn part2(n: &[In]) -> Out {
    Default::default()
}

util::register!(parse, part1, part2);
