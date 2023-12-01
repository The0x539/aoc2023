#![cfg_attr(test, feature(test))]

use util::*;

type N = &'static str;

type In = N;
type Out = i32;

fn parse(s: &'static str) -> In {
    s
}

fn part1(n: &[In]) -> Out {
    let mut s = 0;
    for row in n {
        let a = row.chars().filter(|c| c.is_digit(10)).next().unwrap();
        let b = row.chars().filter(|c| c.is_digit(10)).next_back().unwrap();
        s += p::<i32>(&format!("{a}{b}"));
    }
    s
}

fn a(s: &str) -> i32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!(),
    }
}

fn part2(rows: &[In]) -> Out {
    let mut s = 0;
    let r = regex::Regex::new("[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    for row in rows {
        // damn overlapping regex matches
        let mut digits = vec![];
        let mut i = 0;
        while let Some(m) = r.find(&row[i..]) {
            i += m.start() + 1;
            digits.push(a(m.as_str()));
        }

        s += 10 * digits[0] + digits.last().unwrap();
    }
    s
}

util::register!(parse, part1, part2);
