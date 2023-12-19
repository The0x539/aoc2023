#![cfg_attr(test, feature(test))]

use util::*;

type N = u64;
type Out = N;

type Part = BTreeMap<Field, N>;

struct In {
    flows: BTreeMap<&'static str, Workflow>,
    parts: Vec<Part>,
}

struct Workflow {
    rules: Vec<Rule>,
    fallback: &'static str,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
enum Field {
    X,
    M,
    A,
    S,
}

impl Field {
    fn p(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    field: Field,
    greater: bool,
    n: N,
    to: &'static str,
}

impl Rule {
    fn p(s: &'static str) -> Self {
        let (comp, to) = s.split_once(':').unwrap();
        let field = Field::p(&comp[..1]);
        let greater = &comp[1..2] == ">";
        let n = p(&comp[2..]);
        Self {
            field,
            greater,
            n,
            to,
        }
    }
}

fn parse(s: &'static str) -> In {
    let (a, b) = s.split_once("\n\n").unwrap();

    let mut flows = BTreeMap::default();
    for line in a.lines() {
        let (name, rest) = line.split_once('{').unwrap();
        let mut rules = rest.strip_suffix('}').unwrap().split(',');
        let fallback = rules.next_back().unwrap();
        let rules = rules.map(Rule::p).collect();
        let workflow = Workflow { rules, fallback };
        flows.insert(name, workflow);
    }

    let mut parts = Vec::new();
    for line in b.lines() {
        let mut part = Part::new();
        for foob in line[1..line.len() - 1].split(',') {
            let (field, value) = foob.split_once('=').unwrap();
            part.insert(Field::p(field), p(value));
        }
        parts.push(part);
    }

    In { flows, parts }
}

fn process(part: &Part, flows: &BTreeMap<&str, Workflow>) -> bool {
    let mut key = "in";

    loop {
        let flow = &flows[key];
        let mut new_key = None;
        for rule in &flow.rules {
            let value = part[&rule.field];
            let matched = if rule.greater {
                value > rule.n
            } else {
                value < rule.n
            };
            if matched {
                new_key = Some(rule.to);
                break;
            }
        }
        key = new_key.unwrap_or(flow.fallback);
        if key == "A" {
            return true;
        } else if key == "R" {
            return false;
        }
    }
}

fn part1(n: &In) -> Out {
    n.parts
        .iter()
        .filter(|part| process(part, &n.flows))
        .flat_map(|part| part.values())
        .sum()
}

type R = std::ops::Range<N>;

#[derive(Clone, Debug)]
struct Combo {
    x: R,
    m: R,
    a: R,
    s: R,
}

impl Combo {
    fn len(&self) -> N {
        [&self.x, &self.m, &self.a, &self.s]
            .iter()
            .map(|r| r.end.saturating_sub(r.start))
            .product()
    }

    fn field(&mut self, field: Field) -> &mut R {
        match field {
            Field::X => &mut self.x,
            Field::M => &mut self.m,
            Field::A => &mut self.a,
            Field::S => &mut self.s,
        }
    }

    fn split_field(val: R, rule: &Rule) -> (R, R) {
        let mid = if rule.greater { rule.n + 1 } else { rule.n };
        if rule.greater {
            (mid..val.end, val.start..mid)
        } else {
            (val.start..mid, mid..val.end)
        }
    }

    fn split(mut self, rule: &Rule) -> (Self, Self) {
        let val = self.field(rule.field).clone();
        let (truth, falth) = Self::split_field(val, rule);
        *self.field(rule.field) = truth;
        let left = self.clone();
        *self.field(rule.field) = falth;
        (left, self)
    }
}

fn part2(n: &In) -> Out {
    let everything = Combo {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    let mut states = BTreeMap::new();
    states.insert("in", vec![everything]);

    loop {
        for v in states.values_mut() {
            v.retain(|s| s.len() > 0);
        }
        states.retain(|_, v| !v.is_empty());

        let Some(&flow_name) = states.keys().filter(|k| !matches!(**k, "A" | "R")).next() else {
            break;
        };

        let flow = &n.flows[flow_name];
        for mut state in states.remove(flow_name).unwrap() {
            for rule in &flow.rules {
                let combined_len = state.len();
                let (truth, falth) = state.split(rule);
                assert_eq!(combined_len, truth.len() + falth.len());
                state = falth;
                states.entry(rule.to).or_default().push(truth);
            }
            states.entry(flow.fallback).or_default().push(state);
        }
    }

    states["A"].iter().map(|x| x.len()).sum()
}

util::register!(parse, part1, part2, @alt);
