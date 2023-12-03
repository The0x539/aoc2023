#![cfg_attr(test, feature(test))]

use util::*;

type N = u32;

type In = Vec<Vec<char>>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn part1(grid: &In) -> Out {
    let h = grid.len();
    let w = grid[0].len();

    let mut grid2 = vec![vec!['.'; w]; h];

    for y in 0..h {
        for x in 0..w {
            let c = grid[y][x];
            if c != '.' && !c.is_digit(10) {
                grid2[y][x] = c;
            }
        }
    }

    loop {
        let mut grid3 = grid2.clone();
        for y in 0..h {
            for x in 0..w {
                if grid2[y][x] == '.' {
                    continue;
                }

                let ys = (y.saturating_sub(1))..(h.min(y + 2));
                for yy in ys {
                    let xs = (x.saturating_sub(1))..(w.min(x + 2));
                    for xx in xs {
                        grid3[yy][xx] = grid[yy][xx];
                    }
                }
            }
        }

        if grid3 == grid2 {
            break;
        } else {
            grid2 = grid3;
        }
    }

    grid2
        .into_iter()
        .flat_map(|row| ints::<N>(&row.into_iter().collect::<String>()))
        .sum()
}

fn noom(nums: &mut Vec<N>, mut chars: Vec<char>, rev: bool) {
    if chars.is_empty() {
        return;
    }
    if rev {
        chars.reverse();
    }
    nums.extend(chars.iter().collect::<String>().parse::<N>());
}

fn dg(c: &char) -> bool {
    c.is_digit(10)
}

fn y_neigh(nums: &mut Vec<N>, row: &[char], x: usize) {
    let w = row.len();

    if row[x] == '.' {
        let left_corner = row[..x].iter().copied().rev().take_while(dg).collect();
        noom(nums, left_corner, true);

        let right_corner = row[x + 1..].iter().copied().take_while(dg).collect();
        noom(nums, right_corner, false);
    } else {
        let mut x0 = x;
        while x0 > 0 && dg(&row[x0 - 1]) {
            x0 -= 1;
        }
        let mut x1 = x;
        while x1 < w - 1 && dg(&row[x1 + 1]) {
            x1 += 1;
        }

        noom(nums, row[x0..=x1].into(), false);
    }
}

fn part2(grid: &In) -> Out {
    let h = grid.len();
    let w = grid[0].len();

    let mut sum = 0;

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != '*' {
                continue;
            }

            let mut nums = Vec::new();
            if y > 0 {
                y_neigh(&mut nums, &grid[y - 1], x);
            }
            if y < h - 1 {
                y_neigh(&mut nums, &grid[y + 1], x);
            }
            if x > 0 {
                let left = grid[y][..x].iter().copied().rev().take_while(dg).collect();
                noom(&mut nums, left, true);
            }
            if x < w - 1 {
                let right = grid[y][x + 1..].iter().copied().take_while(dg).collect();
                noom(&mut nums, right, false);
            }

            if nums.len() == 2 {
                sum += nums[0] * nums[1]
            }
        }
    }

    sum
}

util::register!(parse, part1, part2, @alt);
