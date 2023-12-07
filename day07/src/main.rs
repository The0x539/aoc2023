#![cfg_attr(test, feature(test))]

use util::*;

type N = u32;

type In = ([N; 5], N);
type Out = N;

fn parse(s: &'static str) -> In {
    let (hand, bid) = s.split_once(' ').unwrap();
    let hand = hand
        .chars()
        .map(|c| match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '0'..='9' => c as N - '0' as N,
            _ => panic!("{c}"),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let bid = p(bid);
    (hand, bid)
}

fn score(hand: [N; 5]) -> N {
    let mut counts = BTreeMap::<N, N>::new();
    for card in hand {
        *counts.entry(card).or_default() += 1;
    }
    let mut foo = counts.into_iter().collect::<Vec<_>>();
    foo.sort_by_key(|(card, count)| std::cmp::Reverse((*count, *card)));

    if foo.len() == 1 {
        7
    } else if foo[0].1 == 4 {
        6
    } else if foo[0].1 == 3 && foo[1].1 == 2 {
        5
    } else if foo[0].1 == 3 {
        4
    } else if foo[0].1 == 2 && foo[1].1 == 2 {
        3
    } else if foo[0].1 == 2 {
        2
    } else {
        1
    }
}

fn joker_score(hand: [N; 5]) -> N {
    let mut options = vec![hand];
    for i in 0..5 {
        for mut h in std::mem::take(&mut options) {
            if h[i] == 0 {
                for j in 0..=14 {
                    h[i] = j;
                    options.push(h);
                }
            } else {
                options.push(h);
            }
        }
    }
    options.into_iter().map(score).max().unwrap()
}

fn part1(n: &[In]) -> Out {
    let mut hands = n.to_vec();
    hands.sort_by_key(|h| (score(h.0), h.0));

    hands
        .iter()
        .zip(1..)
        .map(|((_cards, bid), rank)| rank * bid)
        .sum()
}

fn part2(n: &[In]) -> Out {
    let mut hands = n.to_vec();
    for (hand, _) in &mut hands {
        for card in hand {
            if *card == 11 {
                *card = 0;
            }
        }
    }
    hands.sort_by_cached_key(|h| (joker_score(h.0), h.0));
    hands
        .iter()
        .zip(1..)
        .map(|((_cards, bid), rank)| rank * bid)
        .sum()
}

util::register!(parse, part1, part2);
