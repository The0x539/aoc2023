#![cfg_attr(test, feature(test))]
#![feature(sort_floats)]

use itertools::Itertools;
use util::*;

type N = i128;

#[derive(Copy, Clone, Debug, PartialEq, Default, Eq, Hash)]
struct Xyz {
    x: N,
    y: N,
    z: N,
}

#[derive(Copy, Clone, PartialEq, Default, Eq, Hash)]
struct Projectile {
    pos: Xyz,
    vel: Xyz,
}

impl Projectile {
    fn advance(&mut self, n: N) {
        self.pos.x += n * self.vel.x;
        self.pos.y += n * self.vel.y;
        self.pos.z += n * self.vel.z;
    }
}

impl Xyz {
    fn sqrlen(&self, other: Xyz) -> u128 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl std::fmt::Debug for Projectile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Xyz { x, y, z } = self.pos;
        write!(f, "({x} {y} {z}")?;
        let Xyz { x, y, z } = self.vel;
        write!(f, " @ {x} {y} {z})")
    }
}

impl std::ops::AddAssign for Xyz {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Add for Xyz {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::SubAssign for Xyz {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Sub for Xyz {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

type In = Projectile;
type Out = usize;

fn parse(s: &'static str) -> In {
    let [a, b, c, d, e, f] = ints_n(s);
    In {
        pos: Xyz { x: a, y: b, z: c },
        vel: Xyz { x: d, y: e, z: f },
    }
}

fn is_future(a: In, x: f64) -> bool {
    (x > a.pos.x as f64) == (a.vel.x > 0)
}

fn to_slope_intercept(a: In) -> (f64, f64) {
    let x = a.pos.x as f64;
    let y = a.pos.y as f64;
    let dx = a.vel.x as f64;
    let dy = a.vel.y as f64;
    let slope = dy / dx;
    let intercept = y - slope * x;
    (slope, intercept)
}

fn part1(n: &[In]) -> Out {
    let range = if cfg!(test) {
        7.0..=27.0
    } else {
        200000000000000.0..=400000000000000.0
    };

    to_slope_intercept(n[0]);

    let mut pairs = HashSet::new();
    for v in n.iter().copied().combinations(2) {
        let (m1, b1) = to_slope_intercept(v[0]);
        let (m2, b2) = to_slope_intercept(v[1]);
        // y = m1 * x + b1
        // y = m2 * x + b2
        // m1 * x + b1 = m2 * x + b2

        // (m1 - m2) * x + b1 = b2
        let m = m1 - m2;
        // (m1 -println m2) * x = b2 - b1
        let b = b2 - b1;
        // x = (b2 - b1) / (m1 - m2)
        let x = b / m;

        let y = m1 * x + b1;

        // sanity check
        let y2 = m2 * x + b2;
        if (y - y2).abs() >= (y / 100000.0).abs() {
            assert_eq!(y, y2);
        }

        if !range.contains(&x) || !range.contains(&y) {
            continue;
        }

        // if !is_future(v[0], x) || !is_future(v[1], x) {
        //     continue;
        // }

        pairs.insert((v[0], v[1]));
    }
    let n = pairs.len();

    let mut counts = HashMap::<In, u32>::new();
    for (a, b) in pairs {
        *counts.entry(a).or_default() += 1;
        *counts.entry(b).or_default() += 1;
    }

    let mut counts = Vec::from_iter(counts);
    counts.sort_by_key(|c| c.1);

    for c in counts {
        println!("{c:?}");
    }

    n
}

fn bbox(stones: &[In]) -> u128 {
    // let mut p0 = Xyz {
    //     x: N::MAX,
    //     y: N::MAX,
    //     z: N::MAX,
    // };
    // let mut p1 = Xyz {
    //     x: N::MIN,
    //     y: N::MIN,
    //     z: N::MIN,
    // };
    // for s in stones {
    //     p0.x = p0.x.min(s.pos.x);
    //     p0.y = p0.y.min(s.pos.y);
    //     p0.z = p0.z.min(s.pos.z);
    //     p1.x = p1.x.max(s.pos.x);
    //     p1.y = p1.y.max(s.pos.y);
    //     p1.z = p1.z.max(s.pos.z);
    // }
    // let width = BigInt::from(p1.x - p0.x);
    // let height = BigInt::from(p1.y - p0.y);
    // let depth = BigInt::from(p1.z - p0.z);
    // // width * height * depth
    // width
    let n = stones
        .iter()
        .copied()
        .combinations(2)
        .map(|pair| pair[0].pos.sqrlen(pair[1].pos))
        .sum();
    n
}

fn part2(input: &[In]) -> Out {
    /*
    the input is 300 trajectories: pᵢˣʸᶻ + t * vᵢˣʸᶻ
    we are solving for a 301st trajectory,  pˣʸᶻ + t * vˣʸᶻ, that intersects with each of them (for 300 different `t` values)

    for a specific choice of i, a choice of p and v will intersect iff there exists a solution t to the equation p + t*v = pᵢ+ t * vᵢ
    that equation refactors to p - pᵢ = t * (vᵢ - v)
    vᵢ != v always, because otherwise the trajectories would never collide
    therefore, "this equation has a solution" iff "p - pᵢ and vᵢ - v are linearly dependent"

    two vectors are linearly dependent iff, when you use them as columns of a matrix, like so:
    ┌                    ┐
    │ pˣ - pᵢˣ, vᵢˣ - vˣ │
    │ pʸ - pᵢʸ, vᵢʸ - vʸ │
    │ pᶻ - pᵢᶻ, vᵢᶻ - vᶻ │
    └                    ┘
    , the matrix has rank 1. this is the case iff, for each of the three possible 2x2 submatrices, the determinant is 0.
    this predicate can be represented as a system of equations:
    (pˣ - pᵢˣ)(vᵢʸ - vʸ) - (vᵢˣ - vˣ)(pʸ - pᵢʸ) = 0                (x and y rows)
    (pʸ - pᵢʸ)(vᵢᶻ - vᶻ) - (vᵢʸ - vʸ)(pᶻ - pᵢᶻ) = 0                (y and z rows)
    (pˣ - pᵢˣ)(vᵢᶻ - vᶻ) - (vᵢˣ - vˣ)(pᶻ - pᵢᶻ) = 0                (x and z rows)
    the t term vanished at some point. now we're solving for pˣʸᶻ and vˣʸᶻ, and we have 300 triplets of equations to constrain our solution

    but first, let's try and make the system a bit more linear
    to avoid typo-prone repetition, let a/b represent "x/y", "y/z", or "x/z"

    (pᵃ - pᵢᵃ)(vᵢᵇ - vᵇ) - (vᵢᵃ - vᵃ)(pᵇ - pᵢᵇ) = 0
    // expand the two products
    (pᵃvᵢᵇ - pᵃvᵇ - pᵢᵃvᵢᵇ + pᵢᵃvᵇ) - (vᵢᵃpᵇ - vᵢᵃpᵢᵇ - vᵃpᵇ + vᵃpᵢᵇ) = 0
    // distribute to remove parens
    pᵃvᵢᵇ - pᵃvᵇ - pᵢᵃvᵢᵇ + pᵢᵃvᵇ - vᵢᵃpᵇ + vᵢᵃpᵢᵇ + vᵃpᵇ - vᵃpᵢᵇ = 0
    // sort terms by degree - constants have subscripts, variables do not
         2                      1                        0
    ┌────┴────┐   ┌─────────────┴─────────────┐   ┌──────┴──────┐
    vᵃpᵇ - pᵃvᵇ + pᵃvᵢᵇ + pᵢᵃvᵇ - vᵢᵃpᵇ - vᵃpᵢᵇ + vᵢᵃpᵢᵇ - pᵢᵃvᵢᵇ = 0
    // we have enough equations that we can "ignore" the degree-2 terms by turning them into their own term:
    qᵃᵇ = vᵃpᵇ - pᵃvᵇ
    qᵃᵇ + pᵃvᵢᵇ + pᵢᵃvᵇ - vᵢᵃpᵇ - vᵃpᵢᵇ + vᵢᵃpᵢᵇ - pᵢᵃvᵢᵇ = 0
    // separate unknowns from coefficients
    ┌                                            ┐
    │ qᵃᵇ, pᵃ , vᵇ ,  pᵇ ,  vᵃ , 1               │
    │ 1  , vᵢᵇ, pᵢᵃ, -vᵢᵃ, -pᵢᵇ, vᵢᵃpᵢᵇ - pᵢᵃvᵢᵇ │
    └                                            ┘
    // return to x/y/z notation
    ┌                                                                  ┐
    │ qˣʸ, qʸᶻ, qˣᶻ, pˣ ,  pʸ ,  pᶻ ,  vˣ ,  vʸ , vᶻ , 1               │
    │ 1  , 0  , 0  , vᵢʸ, -vᵢˣ,  0  , -pᵢʸ,  pᵢˣ, 0  , vᵢˣpᵢʸ - pᵢˣvᵢʸ │
    │ 0  , 1  , 0  , 0  ,  vᵢᶻ, -vᵢʸ,  0  , -pᵢᶻ, pᵢʸ, vᵢʸpᵢᶻ - pᵢʸvᵢᶻ │
    │ 0  , 0  , 1  , vᵢᶻ,  0  , -vᵢˣ, -pᵢᶻ,  0  , pᵢˣ, vᵢˣpᵢᶻ - pᵢˣvᵢᶻ │
    └                                                                  ┘
    */

    0
}

util::register!(parse, part1, part2);
