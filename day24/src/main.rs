#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use itertools::Itertools;
use util::*;

type N = i64;
type In = Projectile;
type Out = N;

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

    let mut count = 0;
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

        if !is_future(v[0], x) || !is_future(v[1], x) {
            continue;
        }

        count += 1;
    }
    count
}

fn part2_mini(input: &[In; 3]) -> Option<(f64, f64, f64)> {
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

    // now we know how to construct a linear system of 900 equations (15 in the test case) with 9 unknowns
    // in theory, we can solve the problem with 9 equations (3 trajectories)
    // let's try it

    let mut equations = vec![];
    for ray in input {
        let p = ray.pos;
        let v = ray.vel;
        let xy = [1, 0, 0, v.y, -v.x, 0, -p.y, p.x, 0, v.x * p.y - p.x * v.y];
        let yz = [0, 1, 0, 0, v.z, -v.y, 0, -p.z, p.y, v.y * p.z - p.y * v.z];
        let xz = [0, 0, 1, v.z, 0, -v.x, -p.z, 0, p.x, v.x * p.z - p.x * v.z];
        equations.extend([xy, yz, xz].map(|eqn| eqn.map(|term| term as f64)));
    }

    // the rounding is a bit of a crapshoot
    row_reduce(&mut equations)?;
    back_substitute(&mut equations);
    reduce_row_echelon(&mut equations);
    round(&mut equations);

    let (_qxy, _qyz, _qxz, px, py, pz, _vx, _vy, _vz) = equations
        .iter()
        .map(|row| row[row.len() - 1])
        .next_tuple()
        .unwrap();

    Some((px, py, pz))
}

fn part2(n: &[In]) -> Out {
    // alternative approach: try the cartesian product of ceil vs. floor for each coordinate
    // it's only 8 combinations - check if that choice of rounding is actually a solution

    // hopefully almost 300 opinions is enough to find an answer that rounded correctly
    for trio in n.array_windows() {
        let Some((px, py, pz)) = part2_mini(trio) else {
            continue;
        };
        // check that rounding errors were small enough to get smoothed out
        if px.fract() == 0.0 && py.fract() == 0.0 && pz.fract() == 0.0 {
            // for some reason my answer comes out negative, but that's not really a problem
            assert_eq!(px.signum(), py.signum());
            assert_eq!(py.signum(), pz.signum());
            return (px as N + py as N + pz as N).abs();
        }
    }
    panic!("well darn");
}

// Get a matrix into triangular form.
fn row_reduce<const N: usize>(matrix: &mut [[f64; N]]) -> Option<()> {
    for i in 0..matrix.len() {
        if matrix[i][i] == 0.0 {
            // oh no, the current "src" row has a 0 diagonal value
            // quick, fix it
            for j in (i + 1)..matrix.len() {
                // rows in matrix[i..] are interchangeable
                // they're all filled with 0 in row[..i]
                // just find one where row[i] is nonzero
                if matrix[j][i] != 0.0 {
                    // and exchange them
                    matrix.swap(i, j);
                    break;
                }
            }
        }

        let src = matrix[i];
        if src[i] == 0.0 {
            // idk, it happens
            return None;
        }

        for j in (i + 1)..matrix.len() {
            let dst = &mut matrix[j];
            if dst[i] == 0.0 {
                continue;
            }
            let ratio = dst[i] / src[i];
            for (d, s) in std::iter::zip(dst, src) {
                *d -= ratio * s;
            }
        }
    }

    Some(())
}

// Get a triangular matrix into row-echelon form.
fn back_substitute<const N: usize>(matrix: &mut [[f64; N]]) {
    for i in (0..matrix.len()).rev() {
        let src = matrix[i];
        assert_ne!(src[i], 0.0);

        for j in 0..i {
            let dst = &mut matrix[j];
            if dst[i] == 0.0 {
                continue;
            }
            let ratio = dst[i] / src[i];
            for (d, s) in std::iter::zip(dst, src) {
                *d -= ratio * s;
            }
        }
    }
}

// Get a row-echelon matrix into reduced row-echelon form.
fn reduce_row_echelon<const N: usize>(matrix: &mut [[f64; N]]) {
    for i in 0..matrix.len() {
        let a = matrix[i][i];
        assert_ne!(a, 0.0);
        for b in &mut matrix[i] {
            *b /= a;
        }
    }
}

// Smooth out floating-point rounding errors.
// Ideally, we'd have done all this work in integer space to begin with,
// but that would require much more checking-for-divisibility than I'd like.
fn round<const N: usize>(matrix: &mut [[f64; N]]) {
    for v in matrix.iter_mut().flatten() {
        let fr = v.fract().abs();
        if fr < 0.000000001 || fr > 0.999999999 {
            *v = v.round();
        }
    }
}

util::register!(parse, part1, part2);

#[cfg(test)]
mod tests {
    use super::*;

    type Mat = [[f64; 4]; 3];

    const INITIAL: Mat = [
        [2.0, 1.0, -1.0, 8.0],
        [-3.0, -1.0, 2.0, -11.0],
        [-2.0, 1.0, 2.0, -3.0],
    ];

    const TRIANGULAR: Mat = [
        [2.0, 1.0, -1.0, 8.0],
        [0.0, 0.5, 0.5, 1.0],
        [0.0, 0.0, -1.0, 1.0],
    ];

    const ECHELON: Mat = [
        [2.0, 0.0, 0.0, 4.0],
        [0.0, 0.5, 0.0, 1.5],
        [0.0, 0.0, -1.0, 1.0],
    ];

    const SOLVED: Mat = [
        [1.0, 0.0, 0.0, 2.0],
        [0.0, 1.0, 0.0, 3.0],
        [0.0, 0.0, 1.0, -1.0],
    ];

    fn check_eq(actual: Mat, expected: Mat) {
        if actual != expected {
            eprintln!("expected:");
            print_matrix(&expected);
            eprintln!("actual:");
            print_matrix(&actual);
            panic!("matrices were not equal");
        }
    }

    #[test]
    fn test_row_reduction() {
        let mut matrix = INITIAL;
        row_reduce(&mut matrix);
        let expected = TRIANGULAR;
        check_eq(matrix, expected);
    }

    #[test]
    fn test_back_substitution() {
        let mut matrix = TRIANGULAR;
        back_substitute(&mut matrix);
        let expected = ECHELON;
        check_eq(matrix, expected);
    }

    #[test]
    fn test_row_echelon_reduction() {
        let mut matrix = ECHELON;
        reduce_row_echelon(&mut matrix);
        let expected = SOLVED;
        check_eq(matrix, expected);
    }
}

pub fn print_matrix<const N: usize>(matrix: &[[f64; N]]) {
    let widths: [usize; N] = std::array::from_fn(|i| {
        matrix
            .iter()
            .map(|row| row[i].to_string().len())
            .max()
            .unwrap()
    });

    print!("┌ ");
    for width in widths {
        print!(" {: <width$}", "");
    }
    println!("┐");

    for row in matrix {
        print!("│ ");
        for (value, width) in std::iter::zip(row, widths) {
            print!("{value:width$} ");
        }
        println!("│");
    }

    print!("└ ");
    for width in widths {
        print!(" {: <width$}", "");
    }
    println!("┘");
}
