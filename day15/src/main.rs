#![cfg_attr(test, feature(test))]

use util::*;

type In = Vec<&'static str>;

fn parse(s: &'static str) -> In {
    s.replace('\n', "").leak().split(',').collect()
}

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |a, c| a.wrapping_add(c).wrapping_mul(17))
}

fn part1(n: &In) -> u32 {
    n.iter().copied().map(hash).map(u32::from).sum()
}

fn part2(n: &In) -> u32 {
    let mut buckets: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    for step in n {
        let label = step.split_once(&['=', '-']).unwrap().0;
        let bucket = &mut buckets[hash(label) as usize];

        let value = step.split_once('=').map(|v| p(v.1));
        let index = bucket.iter().position(|pair| pair.0 == label);

        match (value, index) {
            (Some(v), Some(i)) => bucket[i].1 = v,
            (Some(v), None) => bucket.push((label, v)),
            (None, Some(i)) => _ = bucket.remove(i),
            _ => (),
        }
    }

    let mut s = 0;
    for (bucket, i) in buckets.iter().zip(1..) {
        for (lens, j) in bucket.iter().zip(1..) {
            s += i * j * lens.1;
        }
    }
    s
}

util::register!(parse, part1, part2, @alt);
