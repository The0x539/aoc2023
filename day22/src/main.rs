#![cfg_attr(test, feature(test))]

use slab::Slab;
use std::ops::Add;
use util::*;

type N = i32;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Xyz {
    x: N,
    y: N,
    z: N,
}

impl Add for Xyz {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

#[derive(Copy, Clone, Debug)]
struct Brick {
    a: Xyz,
    b: Xyz,
}

impl Add<Xyz> for Brick {
    type Output = Self;

    fn add(mut self, rhs: Xyz) -> Self::Output {
        self.a = self.a + rhs;
        self.b = self.b + rhs;
        self
    }
}

impl Brick {
    fn iter(&self) -> impl Iterator<Item = Xyz> {
        let x0 = self.a.x.min(self.b.x);
        let y0 = self.a.y.min(self.b.y);
        let z0 = self.a.z.min(self.b.z);
        let x1 = self.a.x.max(self.b.x);
        let y1 = self.a.y.max(self.b.y);
        let z1 = self.a.z.max(self.b.z);
        use std::iter::repeat;
        (x0..=x1)
            .flat_map(move |x| repeat(x).zip(y0..=y1))
            .flat_map(move |xy| repeat(xy).zip(z0..=z1))
            .map(|((x, y), z)| Xyz { x, y, z })
    }
}

type In = Brick;
type Out = usize;

fn parse(s: &'static str) -> In {
    let [a, b, c, d, e, f] = ints_n(s);
    Brick {
        a: Xyz { x: a, y: b, z: c },
        b: Xyz { x: d, y: e, z: f },
    }
}

#[derive(Default, Clone)]
struct World {
    bricks: Slab<Brick>,
    spaces: BTreeMap<Xyz, usize>,
}

impl World {
    fn insert(&mut self, brick: Brick) {
        let id = self.bricks.insert(brick);
        for v in brick.iter() {
            self.spaces.insert(v, id);
        }
    }

    fn settle_once(&mut self) -> bool {
        let mut changed = false;

        'outer: for (id, brick) in &mut self.bricks {
            let potential = *brick + Xyz { x: 0, y: 0, z: -1 };

            for v in potential.iter() {
                if v.z <= 0 {
                    continue 'outer;
                }
                if let Some(&o_id) = self.spaces.get(&v) {
                    if o_id != id {
                        // println!("{id} blocked by {o_id} at {v:?}");
                        continue 'outer;
                    }
                }
            }

            // println!("shifting {id}: {brick:?} -> {potential:?}");

            for v in brick.iter() {
                assert_eq!(self.spaces.remove(&v), Some(id));
            }
            for v in potential.iter() {
                assert_eq!(self.spaces.insert(v, id), None);
            }
            *brick = potential;
            changed = true;
        }

        changed
    }

    fn can_remove(&self, id: usize) -> bool {
        // println!("can_remove {id}");
        let mut supported = BTreeSet::new();

        for v in self.bricks[id].iter() {
            let above = v + Xyz { x: 0, y: 0, z: 1 };
            if let Some(&s_id) = self.spaces.get(&above) {
                if s_id != id {
                    // println!("{id} supports {s_id}");
                    supported.insert(s_id);
                }
            }
        }

        for s_id in supported {
            let mut has_other_supports = false;
            for v in self.bricks[s_id].iter() {
                let below = v + Xyz { x: 0, y: 0, z: -1 };
                if let Some(&b_id) = self.spaces.get(&below) {
                    if b_id != id && b_id != s_id {
                        // println!("{b_id} also supports {s_id}");
                        has_other_supports = true;
                        break;
                    }
                }
            }
            if !has_other_supports {
                // println!("{s_id} is unsupported");
                return false;
            }
        }

        // println!("can INDEED remove {id}");
        true
    }
}

impl std::fmt::Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in (1..=10).rev() {
            for x in 0..=2 {
                let ids = self
                    .spaces
                    .iter()
                    .filter(|(k, _)| k.x == x && k.z == z)
                    .map(|kv| *kv.1)
                    .collect::<BTreeSet<_>>();

                let c = match ids.len() {
                    1 => char::from(b'A' + *ids.iter().next().unwrap() as u8),
                    0 => '.',
                    n => char::from(b'0' + n as u8),
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;

        for z in (1..=10).rev() {
            for y in 0..=2 {
                let ids = self
                    .spaces
                    .iter()
                    .filter(|(k, _)| k.y == y && k.z == z)
                    .map(|kv| *kv.1)
                    .collect::<BTreeSet<_>>();

                let c = match ids.len() {
                    1 => char::from(b'A' + *ids.iter().next().unwrap() as u8),
                    0 => '.',
                    n => char::from(b'0' + n as u8),
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(n: &[In]) -> Out {
    let mut world = World::default();
    for brick in n {
        world.insert(*brick);
    }

    while world.settle_once() {}

    let mut count = 0;
    for (id, _) in &world.bricks {
        if world.can_remove(id) {
            count += 1;
        }
    }
    count
}

fn part2(n: &[In]) -> Out {
    Default::default()
}

util::register!(parse, part1, part2);
