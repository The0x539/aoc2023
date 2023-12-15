#![cfg_attr(test, feature(test))]

use util::*;

type N = usize;

type In = Vec<&'static str>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.replace('\n', "").leak().split(',').collect()
}

fn hash(s: &str) -> N {
    s.chars().fold(0, |a, c| ((a + c as N) * 17) % 256)
}

fn part1(n: &In) -> Out {
    let mut x = 0;
    for step in n {
        x += hash(step);
    }
    x
}

fn part2(n: &In) -> Out {
    let mut buckets: Vec<Vec<(&str, N)>> = vec![vec![]; 256];

    for step in n {
        let label = step.split_once(&['=', '-']).unwrap().0;
        let bucket = &mut buckets[hash(label)];

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
