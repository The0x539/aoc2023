#![cfg_attr(test, feature(test))]

use util::*;

type C3 = [char; 3];
type In = (Vec<bool>, HashMap<C3, (C3, C3)>);
type Out = u64;

fn parse(s: &'static str) -> In {
    let mut lines = s.lines();

    let instructions = lines.next().unwrap().chars().map(|c| c == 'R').collect();
    lines.next();

    let network = lines
        .map(|l| {
            let cs = l
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<Vec<_>>();

            (
                cs[..3].try_into().unwrap(),
                (cs[3..6].try_into().unwrap(), cs[6..].try_into().unwrap()),
            )
        })
        .collect();

    (instructions, network)
}

fn solve(mut v: C3, (instructions, network): &In, f: impl Fn(C3) -> bool) -> Out {
    let mut i = 0;
    loop {
        for &right in instructions {
            i += 1;
            let (l, r) = network[&v];
            v = if right { r } else { l };
            if f(v) {
                return i;
            }
        }
    }
}

fn part1(n: &In) -> Out {
    solve(['A'; 3], n, |v| v == ['Z'; 3])
}

fn part2(n: &In) -> Out {
    n.1.keys()
        .copied()
        .filter(|v| v[2] == 'A')
        .map(|v| solve(v, n, |v| v[2] == 'Z'))
        .reduce(num::integer::lcm)
        .unwrap()
}

util::register!(parse, part1, part2, @alt);
