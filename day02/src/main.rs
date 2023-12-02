#![cfg_attr(test, feature(test))]

use std::collections::HashMap;

use util::*;

type N = u32;

type In = Vec<Vec<(N, &'static str)>>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.split_once(": ")
        .unwrap()
        .1
        .split(';')
        .map(|ss| {
            ss.split(", ")
                .map(|sss| {
                    let (n, c) = sss.trim().split_once(' ').unwrap();
                    (p(n), c)
                })
                .collect()
        })
        .collect()
}

fn part1(n: &[In]) -> Out {
    let mut sum = 0;
    'outer: for (game, id) in n.iter().zip(1..) {
        for round in game {
            for (count, color) in round {
                let max = match *color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!(),
                };
                if *count > max {
                    continue 'outer;
                }
            }
        }
        sum += id;
    }
    sum
}

fn part2(n: &[In]) -> Out {
    let mut sum = 0;
    for game in n {
        let mut reqs = HashMap::new();
        for round in game {
            for (count, color) in round {
                let v = reqs.entry(*color).or_insert(0);
                *v = N::max(*v, *count);
            }
        }
        sum += reqs.values().product::<N>();
    }
    sum.try_into().unwrap()
}

util::register!(parse, part1, part2);
