#![cfg_attr(test, feature(test))]

use std::collections::VecDeque;

use owo_colors::OwoColorize;
use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<Tile>;
type Out = usize;

#[derive(PartialEq, Copy, Clone)]
enum Tile {
    Path,
    Forest,
    Slope((N, N)),
}

fn parse(s: &'static str) -> In {
    s.chars()
        .map(|c| match c {
            '.' => Tile::Path,
            '#' => Tile::Forest,
            'v' => Tile::Slope((0, 1)),
            '>' => Tile::Slope((1, 0)),
            _ => unreachable!(),
        })
        .collect()
}

fn part1(n: &[In]) -> Out {
    let mut hikes = vec![vec![P::new(1, 0)]];
    let w = n[0].len() as N;
    let h = n.len() as N;

    let mut finished_hikes: Vec<Vec<P>> = vec![];

    while !hikes.is_empty() {
        for hike in std::mem::take(&mut hikes) {
            let mut continuing = false;

            let cur = hike[hike.len() - 1];
            if let Tile::Slope(direction) = n[cur.y as usize][cur.x as usize] {
                let step = cur + direction;
                if (0..w).contains(&step.x)
                    && (0..h).contains(&step.y)
                    && n[step.y as usize][step.x as usize] != Tile::Forest
                    && !hike.contains(&step)
                {
                    continuing = true;
                    let mut h = hike.clone();
                    h.push(step);
                    hikes.push(h);
                }
            } else {
                for direction in NESW {
                    let step = cur + direction;
                    if (0..w).contains(&step.x)
                        && (0..h).contains(&step.y)
                        && n[step.y as usize][step.x as usize] != Tile::Forest
                        && !hike.contains(&step)
                    {
                        continuing = true;
                        let mut h = hike.clone();
                        h.push(step);
                        hikes.push(h);
                    }
                }
            }

            if !continuing {
                finished_hikes.push(hike);
            }
        }
    }

    finished_hikes
        .iter()
        .filter(|hike| hike.ends_with(&[P::new(w - 2, h - 1)]))
        .map(|hike| hike.len() - 1)
        .max()
        .unwrap()
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Hike {
    visited: BTreeSet<P>,
    dead_ends: BTreeSet<P>,
    current: P,
}

const NESW: [(N, N); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Hike {
    fn go(&mut self, p: P) {
        self.visited.insert(p);
        self.current = p;
    }

    fn handle_dead_ends(&mut self, n: &[In], poi: P) {
        let mut marked = vec![];
        for d in NESW {
            if self.can_visit(poi + d, n) {
                marked.push(poi + d);
            }
        }

        let mut marked = vec![poi];

        // let mut num = 0;

        while !marked.is_empty() {
            for point in std::mem::take(&mut marked) {
                if self.is_dead_end(n, point) {
                    self.dead_ends.insert(point);
                    // num += 1;
                    for d in NESW {
                        if self.can_visit(point + d, n) {
                            marked.push(point + d);
                        }
                    }
                }
            }
        }

        // if num > 0 {
        // println!("{num} dead ends marked");
        // }
    }

    fn is_dead_end(&self, n: &[In], p: P) -> bool {
        let mut options = 0;
        for d in NESW {
            let pp = p + d;
            if self.can_visit(pp, n) {
                options += 1;
            }
        }
        options <= 1
    }

    fn can_visit(&self, p: P, n: &[In]) -> bool {
        (0..n.len() as N).contains(&p.y)
            && (0..n[0].len() as N).contains(&p.x)
            && n[p.y as usize][p.x as usize] != Tile::Forest
            && !self.visited.contains(&p)
            && !self.dead_ends.contains(&p)
    }
}

fn print_i(i: usize) {
    let s = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let c = s[i] as char;
    let s = c.color(owo_colors::XtermColors::from(i as u8 + 1));
    print!("{s}");
}

fn part2(n: &[In]) -> Out {
    let mut start = Hike::default();
    start.go(P::new(1, 0));
    let mut hikes = VecDeque::from([start]);
    let w = n[0].len() as N;
    let h = n.len() as N;

    if true {
        let mut foo = BTreeMap::new();
        let mut bar = Vec::new();

        for y in 0..h {
            for x in 0..w {
                let p = P { x, y };
                let t = n[p.y as usize][p.x as usize];

                if t != Tile::Forest {
                    print!(" ");
                    continue;
                }

                if let Some(&i) = foo.get(&p) {
                    print_i(i);
                    continue;
                }

                // new group
                let group_id = bar.len();
                let group = {
                    bar.push(BTreeSet::new());
                    &mut bar[group_id]
                };

                let mut unvisited = BTreeSet::from([p]);
                while let Some(q) = unvisited.pop_last() {
                    group.insert(q);
                    assert_eq!(foo.insert(q, group_id), None);
                    for d in NESW {
                        let r = q + d;
                        if (0..h).contains(&r.y)
                            && (0..w).contains(&r.x)
                            && !group.contains(&r)
                            && n[r.y as usize][r.x as usize] == Tile::Forest
                        {
                            unvisited.insert(r);
                        }
                    }
                }
                print_i(group_id);
            }
            println!();
        }

        return 0;
    }

    let mut finished_hikes: BTreeSet<Hike> = Default::default();

    let goal = P::new(w - 2, h - 1);

    while let Some(hike) = hikes.pop_back() {
        if hike.dead_ends.contains(&goal) {
            continue;
        }

        let num_options = NESW
            .into_iter()
            .filter(|&d| hike.can_visit(hike.current + d, n))
            .count();

        if num_options == 0 {
            if hike.current == goal {
                println!("{} {}", hikes.len(), finished_hikes.len());
                finished_hikes.insert(hike);
            }
            continue;
        }

        for direction in NESW {
            let step = hike.current + direction;
            if hike.can_visit(step, n) {
                let mut h = hike.clone();
                let prev = h.current;
                h.go(step);
                if num_options > 1 {
                    h.handle_dead_ends(n, prev);
                }
                hikes.push_back(h);
            }
        }
    }

    for h in &finished_hikes {
        assert!(h.dead_ends.intersection(&h.visited).next().is_none());
    }

    finished_hikes
        .iter()
        .filter(|hike| hike.current == P::new(w - 2, h - 1))
        .map(|hike| hike.visited.len() - 1)
        .max()
        .unwrap()
}

util::register!(parse, part1, part2);
