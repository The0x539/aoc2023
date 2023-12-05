#![cfg_attr(test, feature(test))]

use util::*;

type N = u64;

type In = Almanac;
type Out = N;

struct Almanac {
    seeds: Vec<N>,
    maps: Vec<Map>,
}

#[derive(Debug, Copy, Clone)]
struct Range {
    target: N,
    len: N,
}

#[derive(Debug, Default)]
struct Map {
    header: &'static str,
    ranges: BTreeMap<N, Range>,
}

fn parse(s: &'static str) -> In {
    let mut lines = s.lines();
    let seeds = ints(lines.next().unwrap());
    lines.next().unwrap();
    let mut maps = Vec::new();
    while let Some(header) = lines.next() {
        let mut map = Map::default();
        map.header = header;
        for line in &mut lines {
            if line.is_empty() {
                break;
            } else {
                let row = ints::<N>(line);
                let range = Range {
                    target: row[0],
                    len: row[2],
                };
                map.ranges.insert(row[1], range);
            }
        }
        maps.push(map);
    }
    Almanac { seeds, maps }
}

fn lookup(n: &In, mut num: N) -> N {
    for map in &n.maps {
        for (start, range) in &map.ranges {
            let Some(index) = num.checked_sub(*start) else {
                continue;
            };
            if index < range.len {
                num = range.target + index;
                break;
            }
        }
    }
    num
}

fn part1(n: &In) -> Out {
    let mut min = N::MAX;

    for seed in &n.seeds {
        min = N::min(min, lookup(n, *seed));
    }

    min
}

fn part2(n: &In) -> Out {
    #[derive(Debug, Copy, Clone)]
    struct Span {
        start: N,
        len: N,
    }

    let mut spans = n
        .seeds
        .chunks(2)
        .map(|w| Span {
            start: w[0],
            len: w[1],
        })
        .collect::<Vec<_>>();

    for map in &n.maps {
        let mut next = Vec::new();

        for mut span in spans.iter().copied() {
            for (&range_start, range) in &map.ranges {
                if span.len == 0 {
                    break;
                }

                if range_start > span.start {
                    let catchup_span = Span {
                        start: span.start,
                        len: range_start - span.start,
                    };
                    span.start += catchup_span.len;
                    span.len = span.len.saturating_sub(catchup_span.len);
                    next.push(catchup_span);
                }

                assert!(span.start >= range_start);

                let Some(i) = span.start.checked_sub(range_start) else {
                    continue;
                };

                let Some(amount_available) = range.len.checked_sub(i) else {
                    continue;
                };

                let amount_used = span.len.min(amount_available);

                let mapped_span = Span {
                    start: range.target + i,
                    len: amount_used,
                };
                span.start += amount_used;
                span.len -= amount_used;
                next.push(mapped_span);
            }
            if span.len > 0 {
                next.push(span);
            }
        }

        spans = next;
    }

    spans.into_iter().map(|s| s.start).min().unwrap()
}

util::register!(parse, part1, part2, @alt);
