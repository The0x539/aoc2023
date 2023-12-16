#![cfg_attr(test, feature(test))]

use std::mem;

use rayon::prelude::*;
use util::*;

type N = i32;
type P = Pos<N>;

enum Space {
    Empty,
    Pipe,
    Dash,
    Slash,
    Bslash,
}

type In = Vec<Space>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars()
        .map(|c| match c {
            '.' => Space::Empty,
            '|' => Space::Pipe,
            '-' => Space::Dash,
            '/' => Space::Slash,
            '\\' => Space::Bslash,
            _ => panic!(),
        })
        .collect()
}

#[derive(Default, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct Beam {
    pos: P,
    vel: P,
}

fn simulate(n: &[In], beam: Beam) -> Out {
    let w = n[0].len() as N;
    let h = n.len() as N;

    let mut beams = BTreeSet::from([beam]);
    let mut touched = BTreeSet::new();
    let mut states = BTreeSet::new();

    while !beams.is_empty() {
        for mut b in mem::take(&mut beams) {
            if !(0..w as N).contains(&b.pos.x) || !(0..h as N).contains(&b.pos.y) {
                continue;
            }

            let v = &mut b.vel;

            match n[b.pos.y as usize][b.pos.x as usize] {
                Space::Dash if v.y != 0 => {
                    let z = mem::take(&mut v.y);
                    v.x = z;
                    beams.insert(b);
                    b.vel.x = -z;
                }
                Space::Pipe if v.x != 0 => {
                    let z = mem::take(&mut v.x);
                    v.y = z;
                    beams.insert(b);
                    b.vel.y = -z;
                }
                Space::Slash => {
                    (v.x, v.y) = (-v.y, -v.x);
                }
                Space::Bslash => {
                    (v.x, v.y) = (v.y, v.x);
                }
                _ => {}
            }
            beams.insert(b);
        }

        for mut beam in mem::take(&mut beams) {
            touched.insert(beam.pos);
            beam.pos += beam.vel.into();
            beams.insert(beam);
        }

        if !states.insert((beams.clone(), touched.clone())) {
            break;
        }
    }

    touched.len()
}

fn part1(n: &[In]) -> Out {
    simulate(
        n,
        Beam {
            pos: P { x: 0, y: 0 },
            vel: P { x: 1, y: 0 },
        },
    )
}

fn part2(n: &[In]) -> Out {
    let mut options = vec![];
    let w = n[0].len() as N;
    let h = n.len() as N;
    for x in 0..w {
        options.push(Beam {
            pos: P { x, y: 0 },
            vel: P { x: 0, y: 1 },
        });
        options.push(Beam {
            pos: P { x, y: h - 1 },
            vel: P { x: 0, y: -1 },
        });
    }
    for y in 0..h {
        options.push(Beam {
            pos: P { x: 0, y },
            vel: P { x: 1, y },
        });
        options.push(Beam {
            pos: P { x: w - 1, y },
            vel: P { x: -1, y },
        });
    }

    options
        .into_par_iter()
        .map(|b| simulate(n, b))
        .max()
        .unwrap()
}

util::register!(parse, part1, part2);
