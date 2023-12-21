#![cfg_attr(test, feature(test))]

use util::*;

type N = i64;
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

#[allow(unused)]
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

    // print(&grid, &spots);

    spots.len()
}

type Tile = BTreeSet<P>;

#[derive(Default, Debug)]
struct Memory {
    simulation_cache: BTreeMap<(usize, [usize; 4]), usize>,
    interned_lookup: BTreeMap<Tile, usize>,
    interned_storage: Vec<Tile>,
    hits: usize,
    misses: usize,
}

impl Memory {
    fn intern(&mut self, tile: Tile) -> usize {
        if let Some(existing) = self.interned_lookup.get(&tile) {
            *existing
        } else {
            let key = self.interned_storage.len();
            self.interned_lookup.insert(tile.clone(), key);
            self.interned_storage.push(tile);
            key
        }
    }

    fn simulate(
        &mut self,
        tile_idx: usize,
        neighbor_idxs: [usize; 4],
        grid: &[Vec<bool>],
    ) -> usize {
        if let Some(cached) = self.simulation_cache.get(&(tile_idx, neighbor_idxs)) {
            self.hits += 1;
            return *cached;
        }
        self.misses += 1;

        let h = grid.len() as N;
        let w = grid[0].len() as N;

        let [north, east, south, west] = neighbor_idxs.map(|k| &self.interned_storage[k]);
        let tile = &self.interned_storage[tile_idx];

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

        let key = self.intern(new_tile);
        self.simulation_cache.insert((tile_idx, neighbor_idxs), key);
        key
    }
}

fn print_interned_tiles(tiles: &BTreeMap<P, usize>) {
    let x0 = tiles.keys().map(|k| k.x).min().unwrap();
    let x1 = tiles.keys().map(|k| k.x).max().unwrap();
    let y0 = tiles.keys().map(|k| k.y).min().unwrap();
    let y1 = tiles.keys().map(|k| k.y).max().unwrap();

    for y in y0..=y1 {
        for x in x0..=x1 {
            let Some(tile) = tiles.get(&P { x, y }) else {
                print!(".    ");
                continue;
            };
            print!("{tile:<5}");
        }
        println!();
    }
}

const NESW: [(N, N); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn part2(n: &[In]) -> Out {
    let (start, grid) = setup(n);

    let mut memory = Memory::default();
    let blank = memory.intern(Tile::new());

    let mut tiles: BTreeMap<P, usize> = BTreeMap::new();
    tiles.insert(P::new(0, 0), memory.intern(Tile::from([start])));

    let steps = if cfg!(test) { 1000 } else { 26501365 };
    const CYCLE: N = 262;

    for i in 1..=steps {
        tiles.retain(|_, t| *t != blank);

        for pu in Vec::from_iter(tiles.keys().copied()) {
            for dir in NESW {
                tiles.entry(pu + dir).or_insert(blank);
            }
        }

        let mut new_tiles = BTreeMap::new();
        for (&pu, &tile) in &tiles {
            let neighbors = NESW.map(|dir| *tiles.get(&(pu + dir)).unwrap_or(&blank));
            let new_tile = memory.simulate(tile, neighbors, &grid);
            new_tiles.insert(pu, new_tile);
        }
        tiles = new_tiles;

        tiles.retain(|_, t| *t != blank);

        if i % CYCLE == steps % CYCLE && tiles.len() > 21 {
            print_interned_tiles(&tiles);

            let r1 = tiles.keys().map(|k| k.x).max().unwrap();
            let r2 = {
                let mut r = r1;
                let mut j = i;
                while j != steps {
                    j += CYCLE;
                    r += 2;
                }
                r
            };

            let mut end = BTreeMap::<usize, N>::new();
            let mut insert = |p1| {
                *end.entry(tiles[&p1]).or_default() += 1;
            };

            insert(P::new(r1, 0));
            insert(P::new(-r1, 0));
            insert(P::new(0, r1));
            insert(P::new(0, -r1));

            for _ in 1..=r2 {
                insert(P::new(r1, 1));
                insert(P::new(r1, -1));
                insert(P::new(-r1, 1));
                insert(P::new(-r1, -1));
            }

            for _ in 1..r2 {
                insert(P::new(r1 - 1, 1));
                insert(P::new(r1 - 1, -1));
                insert(P::new(1 - r1, 1));
                insert(P::new(1 - r1, -1));
            }

            let corner_src = tiles[&P::new(0, r1 - 1)];
            let other_src = tiles[&P::new(0, r1 - 2)];
            for x in 0..r2 {
                let mut num_corner = r2 - x;
                let mut num_other = num_corner - 1;

                if x != 0 {
                    num_corner *= 2;
                    num_other *= 2;
                }
                *end.entry(corner_src).or_default() += num_corner;
                *end.entry(other_src).or_default() += num_other;
            }

            return end
                .iter()
                .map(|(k, v)| memory.interned_storage[*k].len() * *v as usize)
                .sum();
        }
    }

    tiles
        .values()
        .map(|k| memory.interned_storage[*k].len())
        .sum()
}

util::register!(parse, part1, part2);
