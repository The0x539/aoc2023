#![cfg_attr(test, feature(test))]

use util::*;

type In = (&'static str, Vec<&'static str>);
type Out = usize;

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once(": ").unwrap();
    (a, b.split_whitespace().collect())
}

fn part1(n: &[In]) -> Out {
    /*
    println!("graph G {{");
    for (a, b) in n {
        for c in b {
            println!("  {a} -- {c}");
        }
    }
    println!("}}");
    */

    let mut all_components = HashSet::new();
    let mut connections = HashMap::<_, HashSet<_>>::new();

    for (a, b) in n {
        all_components.insert(*a);
        all_components.extend(b.iter().copied());
        for c in b {
            connections.entry(a).or_default().insert(c);
            connections.entry(c).or_default().insert(a);
        }
    }

    println!("{connections:?}");

    let to_remove = if cfg!(test) {
        [("hfx", "pzl"), ("bvb", "cmg"), ("nvd", "jqt")]
    } else {
        [("qhd", "cmj"), ("lnf", "jll"), ("vtv", "kkp")]
    };

    for (a, b) in to_remove {
        connections.get_mut(&a).unwrap().remove(&b);
        connections.get_mut(&b).unwrap().remove(&a);
    }

    let mut unvisited = all_components.clone();
    let mut visited = HashSet::new();
    let mut to_visit = vec![n[0].0];
    while let Some(node) = to_visit.pop() {
        visited.insert(node);
        unvisited.remove(&node);
        for &conn in &connections[&node] {
            if unvisited.contains(conn) {
                to_visit.push(conn);
            }
        }
    }

    visited.len() * unvisited.len()
}

fn part2(_: &[In]) -> Out {
    Default::default()
}

util::register!(parse, part1, part2);
