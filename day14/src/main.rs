#![cfg_attr(test, feature(test))]

use util::*;

type N = usize;
type In = Vec<Option<bool>>;
type Out = N;

fn parse(s: &'static str) -> In {
    s.chars()
        .map(|c| match c {
            'O' => Some(true),
            '#' => Some(false),
            '.' => None,
            _ => panic!(),
        })
        .collect()
}

fn shift(grid: &mut [In], cy: usize, cx: usize, dy: usize, dx: usize) {
    if grid[cy][cx] == Some(true) && grid[dy][dx] == None {
        grid[cy][cx] = None;
        grid[dy][dx] = Some(true);
    }
}

fn north(grid: &mut [In]) {
    let w = grid[0].len();
    let h = grid.len();
    for y in 1..h {
        for yy in (1..=y).rev() {
            for x in 0..w {
                shift(grid, yy, x, yy - 1, x);
            }
        }
    }
}

fn west(grid: &mut [In]) {
    let w = grid[0].len();
    let h = grid.len();
    for x in 1..w {
        for xx in (1..=x).rev() {
            for y in 0..h {
                shift(grid, y, xx, y, xx - 1);
            }
        }
    }
}

fn east(grid: &mut [In]) {
    let w = grid[0].len();
    let h = grid.len();
    for x in (0..w - 1).rev() {
        for xx in x..w - 1 {
            for y in 0..h {
                shift(grid, y, xx, y, xx + 1);
            }
        }
    }
}

fn south(grid: &mut [In]) {
    let w = grid[0].len();
    let h = grid.len();
    for y in (0..h - 1).rev() {
        for yy in y..h - 1 {
            for x in 0..w {
                shift(grid, yy, x, yy + 1, x);
            }
        }
    }
}

fn load(grid: &[In]) -> Out {
    let h = grid.len();
    let mut n = 0;
    for (y, row) in grid.iter().enumerate() {
        for c in row {
            if *c == Some(true) {
                n += h - y;
            }
        }
    }
    n
}

fn part1(n: &[In]) -> Out {
    let mut grid = n.to_vec();
    north(&mut grid);
    load(&grid)
}

fn part2(n: &[In]) -> Out {
    let mut grid = n.to_vec();

    let mut seen = HashMap::new();

    let mut i = 0;
    let mut done = false;
    let limit = 1000000000;
    while i < limit {
        north(&mut grid);
        west(&mut grid);
        south(&mut grid);
        east(&mut grid);

        if !done {
            if let Some(j) = seen.insert(grid.clone(), i) {
                done = true;
                let gap = i - j;
                while i + gap < limit {
                    i += gap;
                }
            }
        }
        i += 1;
    }

    load(&grid)
}

util::register!(parse, part1, part2);
