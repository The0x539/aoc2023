#![cfg_attr(test, feature(test))]

use std::collections::VecDeque;

use util::*;

type In = ModuleDef;
type Out = usize;

#[derive(PartialEq)]
enum ModuleKind {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

struct ModuleDef {
    kind: ModuleKind,
    name: &'static str,
    outputs: Vec<&'static str>,
}

fn parse(s: &'static str) -> In {
    let (l, r) = s.split_once(" -> ").unwrap();
    let (kind, name) = if let Some(name) = l.strip_prefix('%') {
        (ModuleKind::FlipFlop, name)
    } else if let Some(name) = l.strip_prefix('&') {
        (ModuleKind::Conjunction, name)
    } else {
        (ModuleKind::Broadcaster, l)
    };
    let outputs = r.split(", ").collect();
    ModuleDef {
        kind,
        name,
        outputs,
    }
}

#[derive(Debug, Clone)]
enum ModuleState {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(BTreeMap<&'static str, bool>),
}

impl ModuleState {
    fn pulse(&mut self, high: bool, source: &str) -> Option<bool> {
        match self {
            ModuleState::Broadcaster => Some(high),
            ModuleState::FlipFlop(state) => {
                if high {
                    None
                } else {
                    *state = !*state;
                    Some(*state)
                }
            }
            ModuleState::Conjunction(inputs) => {
                *inputs.get_mut(source).unwrap() = high;
                Some(!inputs.values().all(|x| *x))
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Module {
    name: &'static str,
    state: ModuleState,
    outputs: Vec<&'static str>,
}

fn setup(n: &[In]) -> HashMap<&'static str, Module> {
    let mut modules = HashMap::new();
    for def in n {
        let state = match def.kind {
            ModuleKind::Broadcaster => ModuleState::Broadcaster,
            ModuleKind::FlipFlop => ModuleState::FlipFlop(false),
            ModuleKind::Conjunction => ModuleState::Conjunction(Default::default()),
        };
        let name = def.name;
        let outputs = def.outputs.clone();
        modules.insert(
            name,
            Module {
                name,
                state,
                outputs,
            },
        );
    }

    // wire up the conjunctions
    for def in n {
        for &output in &def.outputs {
            let Some(module) = modules.get_mut(output) else {
                continue;
            };

            let ModuleState::Conjunction(inputs) = &mut module.state else {
                continue;
            };
            inputs.insert(def.name, false);
        }
    }

    modules
}

fn part1(n: &[In]) -> Out {
    let mut modules = setup(n);

    let mut to_process = VecDeque::new();

    let mut lo_pulses = 0;
    let mut hi_pulses = 0;

    for _ in 0..1000 {
        to_process.push_back(("button", false, "broadcaster"));
        lo_pulses += 1;

        while let Some(pulse) = to_process.pop_front() {
            let (src_module, inbound_value, cur_module) = pulse;

            let Some(module) = modules.get_mut(cur_module) else {
                continue;
            };

            if let Some(outbound_value) = module.state.pulse(inbound_value, src_module) {
                for dst_module in &module.outputs {
                    to_process.push_back((cur_module, outbound_value, dst_module));
                    if outbound_value {
                        hi_pulses += 1;
                    } else {
                        lo_pulses += 1;
                    }
                }
            }
        }
    }
    lo_pulses * hi_pulses
}

fn part2(n: &[In]) -> Out {
    if cfg!(test) {
        return 0;
    }

    let mut modules = setup(n);

    let final_junction = modules
        .values()
        .find(|m| m.outputs.contains(&"rx"))
        .unwrap()
        .name;

    let goal = {
        let ModuleState::Conjunction(inputs) = &modules[final_junction].state else {
            panic!();
        };
        inputs.len()
    };

    let mut pulses = VecDeque::new();
    let mut i = 0;
    let mut periods = HashMap::new();

    loop {
        pulses.push_back(("button", false, "broadcaster"));
        i += 1;

        while let Some(pulse) = pulses.pop_front() {
            let (src_module, inbound_value, cur_module) = pulse;

            if cur_module == final_junction && inbound_value {
                // println!("[{i}] {src_module} -{inbound_value}-> {cur_module}");
                periods.insert(src_module, i);
                if periods.len() == goal {
                    return periods.into_values().reduce(num_integer::lcm).unwrap();
                }
            }

            let Some(module) = modules.get_mut(cur_module) else {
                continue;
            };

            if let Some(outbound_value) = module.state.pulse(inbound_value, src_module) {
                for dst_module in &module.outputs {
                    pulses.push_back((cur_module, outbound_value, dst_module));
                }
            }
        }
    }
}

util::register!(parse, part1, part2);
