#![cfg_attr(test, feature(test))]
#![feature(array_windows)]

use util::*;

type N = i32;
type P = Pos<N>;

struct In {
    dir: Dir,
    amount: N,
    color: &'static str,
}

type Out = usize;

#[derive(Copy, Clone)]
enum Dir {
    L,
    D,
    R,
    U,
}

fn parse(s: &'static str) -> In {
    let parts = s.split_whitespace().collect::<Vec<_>>();
    let dir = match parts[0] {
        "L" => Dir::L,
        "D" => Dir::D,
        "R" => Dir::R,
        "U" => Dir::U,
        _ => panic!(),
    };
    let amount = p(parts[1]);
    let color = &parts[2][2..8];
    In { dir, amount, color }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct SideWall {
    x: N,
    upward: bool,
}

fn solve(n: &[In], get_amount: impl Fn(&In) -> (N, Dir)) -> Out {
    let cross = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut all_walls = BTreeSet::new();
    let mut side_walls = BTreeMap::<N, BTreeSet<SideWall>>::new();

    let mut add = |pos: P, dir: (N, N)| {
        all_walls.insert(pos);
        if dir.1 != 0 {
            let wall = SideWall {
                x: pos.x,
                upward: dir.1 < 0,
            };
            side_walls.entry(pos.y).or_default().insert(wall);
        }
    };

    let mut pos = P::default();
    for inst in n {
        let (amount, dir) = get_amount(inst);
        let dir = cross[dir as usize];

        add(pos, dir);

        for _ in 0..amount {
            pos += dir;
            add(pos, dir);
        }
    }

    let mut interior = 0;

    for (y, row) in side_walls {
        let row = Vec::from_iter(row);

        for [left, right] in row.array_windows() {
            if !(left.upward && !right.upward) {
                continue;
            }

            let x = left.x + 1;
            let x1 = right.x;

            if all_walls.contains(&P { x, y }) {
                continue;
            }

            interior += (x..x1).len() as usize;
        }
    }

    interior + all_walls.len()
}

fn part1(n: &[In]) -> Out {
    solve(n, |inst| (inst.amount, inst.dir))
}

fn part2(n: &[In]) -> Out {
    solve(n, |inst| {
        let amount = N::from_str_radix(&inst.color[..5], 16).unwrap();
        let dir = match &inst.color[5..] {
            "0" => Dir::R,
            "1" => Dir::D,
            "2" => Dir::L,
            "3" => Dir::U,
            _ => panic!(),
        };
        (amount, dir)
    })
}

util::register!(parse, part1, part2);
