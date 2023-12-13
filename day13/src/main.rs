#![cfg_attr(test, feature(test))]

type N = usize;
type In = Vec<Vec<Vec<bool>>>;
type Out = N;

fn parse(s: &'static str) -> In {
    Vec::from_iter(s.lines())
        .split(|line| line.is_empty())
        .map(|lines| {
            lines
                .iter()
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

fn find_reflection(group: &[Vec<bool>], nope: Option<N>) -> Option<N> {
    for y in 1..group.len() {
        if nope == Some(y * 100) {
            continue;
        }

        if group[..y]
            .iter()
            .rev()
            .zip(&group[y..])
            .all(|(a, b)| a == b)
        {
            return Some(y * 100);
        }
    }

    let w = group[0].len();

    for x in 1..w {
        if nope == Some(x) {
            continue;
        }

        if (0..x)
            .rev()
            .zip(x..w)
            .all(|(i, j)| group.iter().all(|r| r[i] == r[j]))
        {
            return Some(x);
        }
    }

    None
}

fn part1(n: &In) -> Out {
    n.iter().map(|x| find_reflection(x, None).unwrap()).sum()
}

fn find_alt_reflection(x: &[Vec<bool>]) -> Option<N> {
    let mut group = x.to_owned();
    let h = group.len();
    let w = group[0].len();
    for y in 0..h {
        for x in 0..w {
            let nope = find_reflection(&group, None).unwrap();
            group[y][x] = !group[y][x];
            if let Some(n) = find_reflection(&group, Some(nope)) {
                return Some(n);
            }
            group[y][x] = !group[y][x];
        }
    }
    None
}

fn part2(n: &In) -> Out {
    n.iter().map(|x| find_alt_reflection(x).unwrap()).sum()
}

util::register!(parse, part1, part2, @alt);
