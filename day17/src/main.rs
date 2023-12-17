#![cfg_attr(test, feature(test))]

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<N>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.bytes().map(|c| c - b'0').map(N::from).collect()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Crucible {
    pos: P,
    dir: P,
    fatigue: N,
}

impl Crucible {
    fn new(pos: P, dir: P) -> Self {
        Self {
            pos,
            dir,
            fatigue: 0,
        }
    }

    fn step(mut self) -> Self {
        self.pos += self.dir.into();
        self.fatigue += 1;
        self
    }

    fn left(mut self) -> Self {
        if self.dir.x > 0 || self.dir.y < 0 {
            (self.dir.x, self.dir.y) = (-self.dir.y, -self.dir.x);
        } else {
            (self.dir.x, self.dir.y) = (self.dir.y, self.dir.x);
        }
        self.fatigue = 0;
        self
    }

    fn right(mut self) -> Self {
        if self.dir.x < 0 || self.dir.y > 0 {
            (self.dir.x, self.dir.y) = (-self.dir.y, -self.dir.x);
        } else {
            (self.dir.x, self.dir.y) = (self.dir.y, self.dir.x);
        }
        self.fatigue = 0;
        self
    }
}

fn solve(n: &[In], f: impl Fn(Crucible) -> Vec<Crucible>) -> Out {
    let w = n[0].len() as N;
    let h = n.len() as N;

    let valid = |p: P| (0..w).contains(&p.x) && (0..h).contains(&p.y);

    let mut visited = BTreeSet::<Crucible>::new();
    let mut unvisited = BTreeSet::new();
    let mut current = Crucible::new(P { x: 0, y: 0 }, P { x: 1, y: 0 });
    let mut distances = BTreeMap::new();
    distances.insert(current, 0);
    unvisited.insert(current);

    loop {
        for neighbor in f(current) {
            if !valid(neighbor.pos) || visited.contains(&neighbor) {
                continue;
            }
            let d_t = distances[&current] + n[neighbor.pos.y as usize][neighbor.pos.x as usize];
            let e = distances.entry(neighbor).or_insert(N::MAX);
            unvisited.insert(neighbor);
            *e = N::min(*e, d_t);
        }
        visited.insert(current);
        unvisited.remove(&current);

        /*
        if visited.len() % 1000 == 0 {
            println!("{}", visited.len());
        }
        */

        if unvisited.is_empty() {
            break;
        } else {
            current = unvisited
                .iter()
                .min_by_key(|v| distances[v])
                .cloned()
                .unwrap();
        }
    }

    distances
        .iter()
        .filter(|kv| kv.0.pos == P { x: w - 1, y: h - 1 })
        .map(|kv| *kv.1)
        .min()
        .unwrap()
}

fn part1(n: &[In]) -> Out {
    solve(n, |current| {
        let mut v = vec![current.left().step(), current.right().step()];
        if current.fatigue < 3 {
            v.push(current.step());
        }
        v
    })
}

// takes a few minutes in release mode
fn part2(n: &[In]) -> Out {
    solve(n, |current| {
        let mut v = Vec::new();
        if current.fatigue < 10 {
            v.push(current.step());
        }
        if current.fatigue >= 4 {
            v.push(current.left().step());
            v.push(current.right().step());
        }
        v
    })
}

util::register!(parse, part1, part2);
