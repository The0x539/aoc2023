#![cfg_attr(test, feature(test))]

use util::*;

type N = usize;

type In = (HashSet<N>, HashSet<N>);
type Out = usize;

fn parse(s: &'static str) -> In {
    let (_, s) = s.split_once(':').unwrap();
    let (l, r) = s.split_once('|').unwrap();
    (ints_g(l), ints_g(r))
}

fn part1(n: &[In]) -> Out {
    n.iter()
        .map(|(win, have)| {
            let n = win.intersection(have).count();
            if n == 0 {
                0
            } else {
                2_usize.pow(n as u32 - 1)
            }
        })
        .sum()
}

fn part2(n: &[In]) -> Out {
    let mut counts = vec![1; n.len()];

    for (i, (win, have)) in n.iter().enumerate() {
        let card_strength = counts[i];
        let win_strength = win.intersection(&have).count();
        for j in (i + 1..).take(win_strength) {
            counts[j] += card_strength;
        }
    }

    counts.iter().sum()
}

util::register!(parse, part1, part2);
