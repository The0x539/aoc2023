#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use util::*;

type N = i32;

type In = Vec<N>;
type Out = N;

fn parse(s: &'static str) -> In {
    ints(s)
}

fn common(row: &[N]) -> Vec<Vec<N>> {
    let mut rows = vec![row.to_vec()];
    loop {
        let mut next = rows
            .last()
            .unwrap()
            .array_windows()
            .map(|[a, b]| b - a)
            .collect::<Vec<_>>();

        let done = next.iter().all(|v| *v == 0);
        if done {
            next.push(0);
        }
        rows.push(next);
        if done {
            return rows;
        }
    }
}

fn part1(n: &[In]) -> Out {
    n.iter()
        .map(|row| {
            let mut rows = common(row);

            while rows.len() >= 2 {
                let next = rows.pop().unwrap();
                let cur = rows.last_mut().unwrap();
                cur.push(*cur.last().unwrap() + *next.last().unwrap());
            }

            *rows[0].last().unwrap()
        })
        .sum()
}

fn part2(n: &[In]) -> Out {
    n.iter()
        .map(|row| {
            let mut rows = common(row);

            while rows.len() >= 2 {
                let next = rows.pop().unwrap();
                let cur = rows.last_mut().unwrap();
                cur.insert(0, cur[0] - next[0]);
            }

            rows[0][0]
        })
        .sum()
}

util::register!(parse, part1, part2);
