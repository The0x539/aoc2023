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

    fn walk(&mut self, edge: &Edge) {
        self.current = *edge.last().unwrap();
        self.visited.insert(self.current);
        self.len += edge.len();
    }
}

const NESW: [(N, N); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

struct World<'a> {
    map: &'a [Vec<Tile>],
}

impl World<'_> {
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

    fn dimensions(&self) -> P {
        let h = self.map.len() as N;
        let w = self.map[0].len() as N;
        P::new(w, h)
    }
}

type Edge = Vec<P>;
type Graph = BTreeMap<P, Vec<Edge>>;

fn build_graph(world: &World, origin: P, part2: bool) -> Graph {
    let mut graph = Graph::new();

    let mut nodes = vec![origin];
    while let Some(start) = nodes.pop() {
        for direction in NESW {
            visit_edge(world, &mut graph, &mut nodes, start, direction, part2);
        }
    }

    graph
}

fn visit_edge(
    world: &World,
    graph: &mut Graph,
    nodes: &mut Vec<P>,
    start: P,
    dir: (N, N),
    part2: bool,
) {
    let mut prev = start;
    let mut current = start + dir;

    let Some(tile) = world.try_get(current) else {
        return;
    };

    let mut directed = false;
    match tile {
        Tile::Forest => return,
        Tile::Path => {}
        Tile::Slope(slope) => {
            directed = true;
            if slope != dir && !part2 {
                // wrong direction
                return;
            }
        }
    }

    if let Some(edges) = graph.get(&start) {
        if edges.iter().any(|e| e[0] == current) {
            // this edge has already been visited
            return;
        }
    }

    let mut edge = vec![current];
    while let Ok(next) = world.options(current).filter(|o| *o != prev).exactly_one() {
        prev = current;
        current = next;
        edge.push(current);
    }

    graph.entry(start).or_default().push(edge.clone());
    if !directed {
        edge.pop();
        edge.reverse();
        edge.push(start);
        graph.entry(current).or_default().push(edge);
    }
    nodes.push(current);
}

fn solve(map: &[In], part2: bool) -> Out {
    let world = World { map };
    let start = P::new(1, 0);
    let goal = world.dimensions() + (-2, -1);
    let graph = build_graph(&world, start, part2);

    let mut finished_hikes = vec![];
    let mut ongoing_hikes = vec![Hike::new(start)];

    while !ongoing_hikes.is_empty() {
        for hike in std::mem::take(&mut ongoing_hikes) {
            if hike.current == goal {
                finished_hikes.push(hike);
                continue;
            }

            for edge in &graph[&hike.current] {
                let next = *edge.last().unwrap();
                if !hike.visited.contains(&next) {
                    let mut h = hike.clone();
                    h.walk(edge);
                    ongoing_hikes.push(h);
                }
            }
        }
    }

    finished_hikes.into_iter().map(|h| h.len).max().unwrap()
}

fn part1(n: &[In]) -> Out {
    solve(n, false)
}

fn part2(n: &[In]) -> Out {
    solve(n, true)
}

util::register!(parse, part1, part2);
