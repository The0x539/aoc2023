#![cfg_attr(test, feature(test))]

use std::sync::Arc;

use itertools::Itertools;
use owo_colors::OwoColorize;
use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<Tile>;
type Out = usize;

#[derive(PartialEq, Copy, Clone, PartialOrd, Ord, Eq, Hash)]
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

#[derive(Clone)]
struct Hike {
    visited: BTreeSet<P>,
    no_go: BTreeSet<P>,
    map: Arc<[Vec<Tile>]>,
    current: P,
}

const NESW: [(N, N); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Hike {
    fn go(&mut self, p: P) {
        self.visited.insert(p);
        self.no_go.insert(p);
        self.current = p;
    }

    fn iter_options(&self, p: P) -> impl Iterator<Item = P> + '_ {
        NESW.into_iter()
            .map(move |d| p + d)
            .filter(|pd| self.can_visit(*pd))
    }

    fn handle_dead_ends(&mut self, poi: P) {
        let mut marked = vec![];
        marked.extend(self.iter_options(poi));

        while !marked.is_empty() {
            for point in std::mem::take(&mut marked) {
                if self.is_dead_end(point) {
                    self.no_go.insert(point);
                    marked.extend(self.iter_options(point));
                }
            }
        }
    }

    fn is_dead_end(&self, p: P) -> bool {
        self.iter_options(p).count() <= 1
    }

    fn can_visit(&self, p: P) -> bool {
        let h = self.map.len() as N;

        let w = self.map[0].len() as N;
        (0..h).contains(&p.y)
            && (0..w).contains(&p.x)
            && self.map[p.y as usize][p.x as usize] != Tile::Forest
            && !self.no_go.contains(&p)
    }

    fn key(&self) -> (BTreeSet<P>,) {
        (self.no_go.clone(),)
    }
}

struct World {
    map: Vec<Vec<Tile>>,
}

impl World {
    fn enumerate_tiles(&self) -> impl Iterator<Item = (P, Tile)> + '_ {
        let h = self.map.len();
        let w = self.map[0].len();
        (0..h)
            .flat_map(move |y| std::iter::repeat(y).zip(0..w))
            .map(|(y, x)| (P::new(x as N, y as N), self.map[y][x]))
    }

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

#[derive(Clone)]
struct Edge {
    path: Vec<P>,
    is_reversed: bool,
}

impl Edge {
    fn new(p: P) -> Self {
        Self {
            path: vec![p],
            is_reversed: false,
        }
    }

    fn reverse(&mut self, start: P) {
        self.is_reversed = !self.is_reversed;
        self.path.pop();
        self.path.reverse();
        self.path.push(start);
    }

    fn push(&mut self, p: P) {
        self.path.push(p);
    }
}

fn part2(n: &[In]) -> Out {
    let world = World { map: n.into() };

    let mut graph = BTreeMap::<P, Vec<Edge>>::new();

    let mut nodes = vec![P::new(1, 0)];
    while let Some(start) = nodes.pop() {
        for d in NESW {
            if !world.is_path(start + d) {
                continue;
            }

            let mut previous = start;
            let mut current = start + d;

            if let Some(foo) = graph.get(&start) {
                if foo.iter().any(|e| e.path[0] == current) {
                    continue;
                }
            }

            let mut edge = Edge::new(current);

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

            edge.reverse(start);
            graph.entry(current).or_default().push(edge);

            nodes.push(current);
        }
    }

    println!(
        "{} nodes, {} edges",
        graph.len(),
        graph.values().flatten().count()
    );

    let mut node_colors = HashMap::new();
    let mut edge_colors = HashMap::new();
    let mut i = 0u8;
    for (endpoint, edges) in &graph {
        i += 1;
        node_colors.insert(endpoint, owo_colors::XtermColors::from(i));
        for edge in edges {
            i += 1;
            for dot in &edge.path {
                edge_colors.insert(dot, owo_colors::XtermColors::from(i));
            }
        }
    }

    for (p, tile) in world.enumerate_tiles() {
        if p.x == 0 {
            println!();
        }
        if tile == Tile::Forest {
            print!("█");
        } else if let Some(c) = node_colors.get(&p) {
            print!("{}", "▓".color(*c));
        } else if let Some(c) = edge_colors.get(&p) {
            print!("{}", "░".color(*c));
        } else {
            panic!();
        }
    }
    println!();

    let w = n[0].len() as N;
    let h = n.len() as N;
    let goal = P::new(w - 2, h - 1);

    let ids = graph.keys().zip(1..).collect::<HashMap<_, _>>();
    println!("{}", ids[&goal]);

    println!("graph G {{");
    for (endpoint, edges) in &graph {
        for edge in edges {
            if edge.is_reversed {
                continue;
            }
            let other = edge.path.last().unwrap();
            println!(
                "  {} -- {} [label={}]",
                ids[endpoint],
                ids[other],
                edge.path.len()
            );
        }
    }
    println!("}}");

    return 0;

    let mut start = Hike {
        visited: BTreeSet::new(),
        no_go: BTreeSet::new(),
        map: n.into(),
        current: Default::default(),
    };
    start.go(P::new(1, 0));

    let mut hikes = Vec::new();
    hikes.push(start);

    let mut finished_hikes: Vec<Hike> = Default::default();

    while let Some(hike) = hikes.pop() {
        let num_options = NESW
            .into_iter()
            .filter(|&d| hike.can_visit(hike.current + d))
            .count();

        if num_options == 0 {
            if hike.current == goal {
                println!("{} {}", hikes.len(), finished_hikes.len());
                finished_hikes.push(hike);
            }
            continue;
        }

        if hike.no_go.contains(&goal) {
            println!("dead");
            continue;
        }

        for direction in NESW {
            let step = hike.current + direction;
            if hike.can_visit(step) {
                let mut h = hike.clone();
                let prev = h.current;
                h.go(step);
                if num_options > 1 {
                    h.handle_dead_ends(prev);
                }
                hikes.push(h);
            }
        }
    }

    finished_hikes
        .iter()
        .filter(|hike| hike.current == goal)
        .map(|hike| hike.visited.len() - 1)
        .max()
        .unwrap()
}

util::register!(parse, part1, part2);
