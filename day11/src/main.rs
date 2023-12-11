#![cfg_attr(test, feature(test))]

use util::*;

type N = usize;
type P = Pos<N>;

type In = Vec<bool>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars().map(|c| c == '#').collect()
}

fn part1(rows: &[In]) -> Out {
    common(rows, 2)
}

fn part2(rows: &[In]) -> Out {
    common(rows, 1_000_000)
}

fn common(rows: &[In], growth: N) -> Out {
    let w = rows[0].len();
    let h = rows.len();

    let col_gaps = (0..w)
        .filter(|x| rows.iter().all(|row| !row[*x]))
        .collect::<Vec<_>>();

    let row_gaps = (0..h)
        .filter(|y| !rows[*y].contains(&true))
        .collect::<Vec<_>>();

    let mut gals = Vec::new();
    for y in 0..h {
        for x in 0..w {
            if rows[y][x] {
                gals.push(P { y, x });
            }
        }
    }

    let mut sum = 0;

    for i in 0..gals.len() {
        for j in 0..gals.len() {
            if i >= j {
                continue;
            }
            let a = gals[i];
            let b = gals[j];

            let mut dist = 0;
            for x in N::min(a.x, b.x)..N::max(a.x, b.x) {
                dist += if col_gaps.contains(&x) { growth } else { 1 };
            }
            for y in N::min(a.y, b.y)..N::max(a.y, b.y) {
                dist += if row_gaps.contains(&y) { growth } else { 1 };
            }
            sum += dist;
        }
    }

    sum
}

util::register!(parse, part1, part2);
