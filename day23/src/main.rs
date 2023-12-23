#![cfg_attr(test, feature(test))]

use itertools::Itertools;
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

            let directions = match n[cur.y as usize][cur.x as usize] {
                Tile::Slope(d) => vec![d],
                _ => NESW.to_vec(),
            };

            for direction in directions {
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

#[derive(Clone)]
struct Hike {
    current: P,
    visited: HashSet<P>,
    len: usize,
}

impl Hike {
    fn new(current: P) -> Self {
        Self {
            current,
            visited: HashSet::from([current]),
            len: 0,
        }
    }

    fn walk(&mut self, edge: &[P]) {
        self.current = *edge.last().unwrap();
        self.visited.insert(self.current);
        self.len += edge.len();
    }
}

const NESW: [(N, N); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct World {
    map: Vec<Vec<Tile>>,
}

impl World {
    fn try_get(&self, p: P) -> Option<Tile> {
        if p.x < 0 || p.y < 0 {
            return None;
        }
        self.map
            .get(p.y as usize)
            .and_then(|row| row.get(p.x as usize))
            .copied()
    }

    fn is_path(&self, p: P) -> bool {
        matches!(self.try_get(p), Some(Tile::Path | Tile::Slope(_)))
    }

    fn options(&self, p: P) -> impl Iterator<Item = P> + '_ {
        NESW.into_iter()
            .map(move |d| p + d)
            .filter(|p| self.is_path(*p))
    }
}

fn part2(n: &[In]) -> Out {
    let start = P::new(1, 0);
    let goal = P::new(n[0].len() as N - 2, n.len() as N - 1);

    let world = World { map: n.into() };

    let mut graph = BTreeMap::<P, Vec<Vec<P>>>::new();

    let mut nodes = vec![P::new(1, 0)];
    while let Some(start) = nodes.pop() {
        for d in NESW {
            if !world.is_path(start + d) {
                continue;
            }

            let mut previous = start;
            let mut current = start + d;

            if let Some(branches) = graph.get(&start) {
                if branches.iter().any(|e| e[0] == current) {
                    continue;
                }
            }

            let mut edge = vec![current];

            while let Ok(next) = world
                .options(current)
                .filter(|o| *o != previous)
                .exactly_one()
            {
                previous = current;
                current = next;
                edge.push(current);
            }

            graph.entry(start).or_default().push(edge.clone());

            edge.pop();
            edge.reverse();
            edge.push(start);
            graph.entry(current).or_default().push(edge);

            nodes.push(current);
        }
    }

    let mut finished = vec![];
    let mut ongoing = vec![Hike::new(start)];

    while !ongoing.is_empty() {
        for hike in std::mem::take(&mut ongoing) {
            if hike.current == goal {
                finished.push(hike);
                continue;
            }

            for edge in &graph[&hike.current] {
                let next = *edge.last().unwrap();
                if !hike.visited.contains(&next) {
                    let mut h = hike.clone();
                    h.walk(edge);
                    ongoing.push(h);
                }
            }
        }
    }

    finished.into_iter().map(|h| h.len).max().unwrap()
}

util::register!(parse, part1, part2);
