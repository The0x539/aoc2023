#![cfg_attr(test, feature(test))]
#![feature(iter_intersperse)]

use util::*;

type N = i32;
type P = Pos<N>;

type In = Vec<char>;
type Out = usize;

fn parse(s: &'static str) -> In {
    s.chars().collect()
}

fn part1(n: &[In]) -> Out {
    let y = n.iter().position(|r| r.contains(&'S')).unwrap();
    let x = n[y].iter().position(|c| *c == 'S').unwrap();

    let mut pos = P {
        x: x as N,
        y: y as N,
    };
    let mut dir = P { x: 0, y: 1 };
    let mut dist = 0;

    loop {
        pos += dir.into();
        dist += 1;
        match n[pos.y as usize][pos.x as usize] {
            '|' | '-' => {}
            'L' | '7' => dir = P { x: dir.y, y: dir.x },
            'F' | 'J' => {
                dir = P {
                    x: -dir.y,
                    y: -dir.x,
                }
            }
            'S' => break,
            _ => unreachable!(),
        }
    }

    dist / 2
}

fn preproc(n: &[In]) -> Vec<In> {
    let step1 = n
        .iter()
        .map(|row| {
            row.iter()
                .copied()
                .intersperse_with(|| '-')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let w = step1[0].len();
    step1
        .into_iter()
        .intersperse_with(|| vec!['|'; w])
        .collect()
}

fn part2(n: &[In]) -> Out {
    let n = preproc(n);

    let y = n.iter().position(|r| r.contains(&'S')).unwrap();
    let x = n[y].iter().position(|c| *c == 'S').unwrap();

    let mut pos = P {
        x: x as N,
        y: y as N,
    };
    let mut dir = P { x: 0, y: 1 };

    let w = n[0].len();
    let h = n.len();

    const INIT: char = ' ';
    const FILL: char = '░';

    let mut grid = vec![vec![INIT; w]; h];

    loop {
        pos += dir.into();
        grid[pos.y as usize][pos.x as usize] = n[pos.y as usize][pos.x as usize];
        match n[pos.y as usize][pos.x as usize] {
            '|' | '-' => {}
            'L' | '7' => dir = P { x: dir.y, y: dir.x },
            'F' | 'J' => {
                dir = P {
                    x: -dir.y,
                    y: -dir.x,
                }
            }
            'S' => break,
            _ => unreachable!(),
        }
    }

    grid[h / 2][w / 2] = FILL;

    loop {
        let mut changed = false;
        for y in 0..h {
            'foo: for x in 0..w {
                for dy in [0, 1, 2] {
                    let Some(yy) = (y + dy).checked_sub(1).filter(|yy| *yy < h) else {
                        continue;
                    };
                    for dx in [0, 1, 2] {
                        let Some(xx) = (x + dx).checked_sub(1).filter(|xx| *xx < w) else {
                            continue;
                        };

                        if grid[y][x] == INIT && grid[yy][xx] == FILL {
                            grid[y][x] = FILL;
                            changed = true;
                            continue 'foo;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }

    grid = grid
        .into_iter()
        .step_by(2)
        .map(|row| row.into_iter().step_by(2).collect())
        .collect();

    _print(&grid);

    grid.iter().flatten().filter(|c| c == &&FILL).count()
}

fn _print(grid: &[In]) {
    for row in grid {
        for cell in row {
            let ch = match *cell {
                '|' => '│',
                '-' => '─',
                'F' => '┌',
                '7' => '┐',
                'L' => '└',
                'J' => '┘',
                c => c,
            };
            print!("{ch}");
        }
        println!();
    }
}

util::register!(parse, part1, part2);
