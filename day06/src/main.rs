#![cfg_attr(test, feature(test))]

use util::*;

type N = u64;
// type P = Pos<N>;

struct Race {
    time: N,
    distance: N,
}

type In = Vec<Race>;
type Out = N;

fn parse(s: &'static str) -> In {
    let l = s.lines().collect::<Vec<_>>();
    let times = ints(l[0]);
    let distances = ints(l[1]);
    times
        .iter()
        .zip(distances)
        .map(|(t, d)| Race {
            time: *t,
            distance: d,
        })
        .collect()
}

fn part1(n: &In) -> Out {
    let mut result = 1;
    for race in n {
        let mut v = 0;
        for hold in 0..race.time {
            let go = race.time - hold;
            let distance = go * hold;
            if distance > race.distance {
                v += 1;
            }
        }
        result *= v;
    }
    result
}

fn part2(n: &In) -> Out {
    let race_time = n
        .iter()
        .map(|r| r.time.to_string())
        .collect::<String>()
        .parse::<N>()
        .unwrap();

    let race_distance = n
        .iter()
        .map(|r| r.distance.to_string())
        .collect::<String>()
        .parse::<N>()
        .unwrap();

    let mut min = N::MAX;
    for i in 0..race_time {
        if i * (race_time - i) > race_distance {
            min = i;
            break;
        }
    }
    let mut max = 0;
    for i in min..race_time {
        if i * (race_time - i) < race_distance {
            max = i;
            break;
        }
    }

    (min..max).count() as N
}

util::register!(parse, part1, part2, @alt);
