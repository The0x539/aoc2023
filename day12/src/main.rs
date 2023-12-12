#![cfg_attr(test, feature(test))]
#![feature(iter_intersperse)]

use util::*;

type N = usize;

type In = (Vec<Option<bool>>, Vec<N>);
type Out = usize;

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once(' ').unwrap();
    let a = a
        .chars()
        .map(|c| match c {
            '#' => Some(true),
            '.' => Some(false),
            '?' => None,
            _ => panic!(),
        })
        .collect();
    let b = ints(b);
    (a, b)
}

fn rle(springs: &[bool]) -> Vec<usize> {
    springs
        .split(|damaged| !damaged)
        .filter(|run| !run.is_empty())
        .map(|run| run.len())
        .collect()
}

fn counts(row: &In) -> usize {
    let mut slots = vec![];
    let mut buf = vec![];

    for (i, v) in row.0.iter().copied().enumerate() {
        match v {
            Some(b) => buf.push(b),
            None => {
                buf.push(false);
                slots.push(i);
            }
        }
    }

    let mut options = 0;

    for pattern in 0..(2_u32.pow(slots.len() as u32)) {
        for i in 0..slots.len() {
            buf[slots[i]] = pattern & (1 << i) != 0;
        }

        if rle(&buf) == row.1 {
            options += 1;
        }
    }

    options
}

fn part1(n: &[In]) -> Out {
    n.iter().map(counts).sum()
}

fn counts_opt(row: &In) -> usize {
    let mut bufs = vec![(1, vec![])];

    for spring in &row.0 {
        assert_ne!(bufs.len(), 0);

        for (count, mut buf) in std::mem::take(&mut bufs) {
            if let Some(val) = *spring {
                buf.push(val);
                bufs.push((count, buf));
            } else {
                let mut alt = buf.clone();
                buf.push(false);
                alt.push(true);
                bufs.push((count, buf));
                bufs.push((count, alt));
            }
        }

        let mut counts_by_rle = BTreeMap::new();
        let mut bufs_by_rle = BTreeMap::new();
        for (count, buf) in std::mem::take(&mut bufs) {
            if buf.last() == Some(&true) {
                bufs.push((count, buf));
                continue;
            }

            let runs = rle(&buf);
            if !row.1.starts_with(&runs) {
                continue;
            }

            *counts_by_rle.entry(runs.clone()).or_default() += count;
            bufs_by_rle.insert(runs, buf);
        }
        bufs.extend(counts_by_rle.into_values().zip(bufs_by_rle.into_values()));
    }

    bufs.retain(|(_, buf)| rle(buf) == row.1);
    bufs.into_iter().map(|(count, _buf)| count).sum()
}

fn part2(n: &[In]) -> Out {
    n.iter()
        .map(|(springs, runs)| {
            let mut springs = springs.to_vec();
            let mut runs = runs.to_vec();
            let a = springs.len();
            let b = runs.len();
            for _ in 0..4 {
                springs.push(None);
                springs.extend_from_within(..a);
                runs.extend_from_within(..b);
            }
            counts_opt(&(springs, runs))
        })
        .sum()
}

util::register!(parse, part1, part2);

#[cfg(test)]
#[test]
fn test_rle() {
    let (springs, _) = parse(".#.###.#.###### 1,3,1,6");
    let springs = springs.into_iter().map(Option::unwrap).collect::<Vec<_>>();
    let runs = rle(&springs);
    assert_eq!(runs, vec![1, 3, 1, 6]);
}
