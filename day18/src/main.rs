#![cfg_attr(test, feature(test))]

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

fn solve(n: &[In], get_amount: impl Fn(&In) -> (N, Dir)) -> Out {
    let cross = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    // let mut x0 = N::MAX;
    // let mut y0 = N::MAX;
    // let mut x1 = N::MIN;
    // let mut y1 = N::MIN;

    let mut dug = BTreeSet::new();
    let mut pos = P::default();
    dug.insert(pos);
    for inst in n {
        let (amount, dir) = get_amount(inst);
        let dir = cross[dir as usize];
        for _ in 0..amount {
            pos += dir;
            dug.insert(pos);
        }
        // x0 = x0.min(pos.x);
        // y0 = y0.min(pos.y);
        // x1 = x1.max(pos.x);
        // y1 = y1.max(pos.y);
    }

    let mut interior = HashSet::new();
    let mut to_visit = HashSet::new();
    to_visit.insert(P::new(1, 1));

    loop {
        // let before = inside.len();
        let mut changed = false;
        for spot in std::mem::take(&mut to_visit) {
            if !dug.contains(&spot) && interior.insert(spot) {
                for offset in cross {
                    to_visit.insert(spot + offset);
                }
                changed = true;
            }
        }
        if !changed {
            break;
        }
        // let after = inside.len();
        // let n = 1000000;
        // if before / n < after / n {
        //     println!("{}", inside.len());
        // }
    }

    dug.len() + interior.len()
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
