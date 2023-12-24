#![cfg_attr(test, feature(test))]

use itertools::Itertools;
use util::*;

type N = f64;

#[derive(Copy, Clone, Debug)]
struct Xyz {
    x: N,
    y: N,
    z: N,
}

type In = (Xyz, Xyz);
type Out = usize;

fn parse(s: &'static str) -> In {
    let [a, b, c, d, e, f] = ints_n(s);
    (Xyz { x: a, y: b, z: c }, Xyz { x: d, y: e, z: f })
}

fn is_future(a: In, x: N) -> bool {
    // println!("{a:?} {x}");
    (x > a.0.x) == (a.1.x > 0.0)
}

fn to_slope_intercept(a: In) -> (N, N) {
    let (Xyz { x, y, .. }, Xyz { x: dx, y: dy, .. }) = a;
    let slope = dy / dx;
    let intercept = y - slope * x;
    (slope, intercept)
}

fn part1(n: &[In]) -> Out {
    let range = if cfg!(test) {
        7.0..=27.0
    } else {
        200000000000000.0..=400000000000000.0
    };

    to_slope_intercept(n[0]);

    let mut count = 0;
    for v in n.iter().copied().combinations(2) {
        let (m1, b1) = to_slope_intercept(v[0]);
        let (m2, b2) = to_slope_intercept(v[1]);
        // y = m1 * x + b1
        // y = m2 * x + b2
        // m1 * x + b1 = m2 * x + b2

        // (m1 - m2) * x + b1 = b2
        let m = m1 - m2;
        // (m1 -println m2) * x = b2 - b1
        let b = b2 - b1;
        // x = (b2 - b1) / (m1 - m2)
        let x = b / m;

        let y = m1 * x + b1;

        // sanity check
        let y2 = m2 * x + b2;
        if (y - y2).abs() >= (y / 100000.0).abs() {
            assert_eq!(y, y2);
        }

        if !range.contains(&x) || !range.contains(&y) {
            continue;
        }

        if !is_future(v[0], x) || !is_future(v[1], x) {
            continue;
        }

        count += 1;
    }
    count
}

fn part2(n: &[In]) -> Out {
    Default::default()
}

util::register!(parse, part1, part2);
