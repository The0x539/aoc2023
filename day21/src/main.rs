#![cfg_attr(test, feature(test))]

use std::rc::Rc;

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<Spot>;
type Out = usize;

#[derive(PartialEq)]
enum Spot {
    Dot,
    Hash,
    S,
}

fn parse(s: &'static str) -> In {
    s.chars()
        .map(|c| match c {
            '.' => Spot::Dot,
            '#' => Spot::Hash,
            'S' => Spot::S,
            _ => panic!(),
        })
        .collect()
}

fn setup(n: &[In]) -> (P, Vec<Vec<bool>>) {
    let grid = n
        .iter()
        .map(|row| {
            row.iter()
                .map(|spot| *spot == Spot::Hash)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (row, y) in n.iter().zip(0..) {
        for (spot, x) in row.iter().zip(0..) {
            if *spot == Spot::S {
                return (P { x, y }, grid);
            }
        }
    }

    panic!()
}

fn print(grid: &[Vec<bool>], spots: &HashSet<P>) {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let p = P {
                x: x as _,
                y: y as _,
            };
            if *c {
                print!("#");
            } else if spots.contains(&p) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(n: &[In]) -> Out {
    let (start, grid) = setup(n);

    let w = grid[0].len();
    let h = grid.len();

    let mut spots = HashSet::new();
    spots.insert(start);

    let steps = if cfg!(test) { 6 } else { 64 };

    for _ in 0..steps {
        for spot in std::mem::take(&mut spots) {
            for (dx, dy) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let pos = spot + (dx, dy);
                if !(0..w).contains(&(pos.x as usize)) || !(0..h).contains(&(pos.y as usize)) {
                    continue;
                }
                if grid[pos.y as usize][pos.x as usize] {
                    continue;
                }
                spots.insert(pos);
            }
        }
    }

    print(&grid, &spots);

    spots.len()
}

/*
const NESW: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn simulate_local(
    tile: &BTreeSet<P>,
    [north, east, south, west]: &Neighbors,
    grid: &[Vec<bool>],
) -> BTreeSet<P> {
    let h = grid.len() as N;
    let w = grid[0].len() as N;

    let mut new_tile = BTreeSet::new();

    for y in 0..h {
        for x in 0..w {
            if grid[y as usize][x as usize] {
                continue;
            }

            let p0 = P::new(x, y);
            for (dx, dy) in NESW {
                let p1 = p0 + (dx, dy);

                let neighbor = if p1.x == -1 {
                    west.contains(&P::new(w - 1, p1.y))
                } else if p1.x == w {
                    east.contains(&P::new(0, p1.y))
                } else if p1.y == -1 {
                    north.contains(&P::new(p1.x, h - 1))
                } else if p1.y == h {
                    south.contains(&P::new(p1.x, 0))
                } else {
                    tile.contains(&p1)
                };

                if neighbor {
                    new_tile.insert(p0);
                    break;
                }
            }
        }
    }

    new_tile
}

#[allow(unused)]
fn print_tiles(tiles: &BTreeMap<P, TileRef>) {
    let x0 = tiles.keys().map(|k| k.x).min().unwrap();
    let x1 = tiles.keys().map(|k| k.x).max().unwrap();
    let y0 = tiles.keys().map(|k| k.y).min().unwrap();
    let y1 = tiles.keys().map(|k| k.y).max().unwrap();

    let mut seen = BTreeMap::new();

    for y in y0..=y1 {
        for x in x0..=x1 {
            let pu = P { x, y };
            let Some(tile) = tiles.get(&pu) else {
                print!(".\t");
                continue;
            };

            let i = match seen.get(tile) {
                Some(j) => *j,
                None => {
                    let j = seen.len();
                    seen.insert(tile.clone(), j);
                    j
                }
            };
            print!("{i}\t");
        }
        println!();
    }
}

type TileRef = Rc<BTreeSet<P>>;
type Neighbors = [TileRef; 4];

fn part2(n: &[In]) -> Out {
    let (start, grid) = setup(n);

    let empty = TileRef::default();
    let mut tiles: BTreeMap<P, TileRef> = BTreeMap::new();
    tiles.insert(P::new(0, 0), Rc::new(BTreeSet::from([start])));

    let mut cache: BTreeMap<TileRef, BTreeMap<Neighbors, TileRef>> = BTreeMap::new();

    let steps = if cfg!(test) { 1000 } else { 26501365 };

    let mut last_grow = 0;

    for i in 0..steps {
        tiles.retain(|_, t| !t.is_empty());
        let old_len = tiles.len();

        for pu in Vec::from_iter(tiles.keys().copied()) {
            for dir in NESW {
                tiles.entry(pu + dir).or_default();
            }
        }

        let mut new_tiles = BTreeMap::new();
        for (&pu, tile) in &tiles {
            let neighbors = NESW.map(|dir| tiles.get(&(pu + dir)).unwrap_or(&empty).clone());

            if let Some(cached) = cache.get(tile).and_then(|c| c.get(&neighbors)) {
                new_tiles.insert(pu, cached.clone());
            } else {
                let new_tile = Rc::new(simulate_local(tile, &neighbors, &grid));
                cache
                    .entry(tile.clone())
                    .or_default()
                    .insert(neighbors, new_tile.clone());
                new_tiles.insert(pu, new_tile);
            }
        }

        tiles = new_tiles;
        tiles.retain(|_, t| !t.is_empty());
        let new_len = tiles.len();

        // should look the same as last time, but with one more layer of "inner body"
        if new_len > old_len {
            println!("grew at {i} ({} since last)", i - last_grow);
            last_grow = i;
        }
    }

    tiles.values().map(|t| t.len()).sum()
}
*/
fn part2(n: &[In]) -> Out {
    let (start, grid) = setup(n);

    let mut tile = HashSet::new();
    let steps = 64;
    for x in 0..steps {
        for y in 0..(steps - x) {
            tile.insert(start + (x, y));
        }
    }

    for (row, y) in grid.iter().zip(0..) {
        for (v, x) in row.iter().zip(0..) {
            if grid[y as usize][x as usize] {
                print!("#");
            } else if tile.contains(&P { x, y }) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }

    0
}

util::register!(parse, part1, part2);
