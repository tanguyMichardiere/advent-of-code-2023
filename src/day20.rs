use std::collections::HashMap;
use std::str::FromStr;

use crate::cache::hash;
use crate::regex;

struct State {
    modules: HashMap<u64, Module>,
    low_pulse_count: usize,
    high_pulse_count: usize,
    before_rx: Option<u64>,
    // frequency at which the module before rx receives a high pulse from each of its input modules
    cycles: HashMap<u64, Option<usize>>,
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modules =
            regex!(r"(?P<name>broadcaster|(?:[%&][a-z]+)) -> (?P<outputs>(?:[a-z]+, )*[a-z]+)")
                .captures_iter(s)
                .map(|caps| {
                    let outputs = regex!(r"(?P<name>[a-z]+)")
                        .captures_iter(&caps["outputs"])
                        .map(|caps| hash(&caps["name"]))
                        .collect();
                    match &caps["name"] {
                        "broadcaster" => {
                            let id = crate::cache::hash("broadcaster");
                            (id, Module::Broadcaster(BroadcasterModule { id, outputs }))
                        }
                        name if name.starts_with("%") => {
                            let id = crate::cache::hash(&name[1..]);
                            (
                                id,
                                Module::FlipFlop(FlipFlopModule {
                                    id,
                                    on: false,
                                    outputs,
                                }),
                            )
                        }
                        name if name.starts_with("&") => {
                            let id = crate::cache::hash(&name[1..]);
                            (
                                id,
                                Module::Conjunction(ConjunctionModule {
                                    id,
                                    memory: HashMap::new(),
                                    outputs,
                                }),
                            )
                        }
                        _ => unreachable!(),
                    }
                })
                .collect::<HashMap<_, _>>();
        let mut conjunction_inputs = HashMap::new();
        for (id, module) in &modules {
            for output in module.outputs() {
                if let Some(Module::Conjunction(_)) = modules.get(output) {
                    conjunction_inputs
                        .entry(*output)
                        .or_insert(Vec::new())
                        .push(*id);
                }
            }
        }
        for (name, inputs) in conjunction_inputs {
            if let Module::Conjunction(conjunction) = modules.get_mut(&name).unwrap() {
                for input in inputs {
                    conjunction.memory.insert(input, false);
                }
            }
        }
        let before_rx = modules
            .iter()
            .find_map(|(id, module)| {
                if module.outputs().iter().any(|output| output == &hash("rx")) {
                    Some(id)
                } else {
                    None
                }
            })
            .cloned();
        let cycle_lens = if let Some(before_rx) = before_rx {
            modules
                .keys()
                .filter_map(|id| {
                    if modules[id]
                        .outputs()
                        .iter()
                        .any(|output| output == &before_rx)
                    {
                        Some((*id, None))
                    } else {
                        None
                    }
                })
                .collect::<HashMap<_, _>>()
        } else {
            HashMap::new()
        };
        Ok(Self {
            modules,
            low_pulse_count: 0,
            high_pulse_count: 0,
            before_rx,
            cycles: cycle_lens,
        })
    }
}

impl State {
    fn push_button(&mut self, count: usize) {
        self.resolve(
            vec![(
                Pulse {
                    from: 0,
                    high: false,
                },
                vec![hash("broadcaster")],
            )],
            count,
        );
    }

    fn resolve(&mut self, pulses: Vec<(Pulse, Vec<u64>)>, count: usize) {
        let mut next_pulses = Vec::new();
        for (pulse, outputs) in pulses {
            if pulse.high {
                self.high_pulse_count += outputs.len();
            } else {
                self.low_pulse_count += outputs.len();
            }
            for output in outputs {
                if self.before_rx.is_some_and(|before_rx| output == before_rx) && pulse.high {
                    *self.cycles.get_mut(&pulse.from).unwrap() = Some(count);
                }
                if let Some(module) = self.modules.get_mut(&output) {
                    if let Some(pulse) = module.resolve(&pulse) {
                        next_pulses.push((pulse, module.outputs().to_vec()));
                    }
                }
            }
        }
        if !next_pulses.is_empty() {
            self.resolve(next_pulses, count);
        }
    }

    fn cycles_to_activate(&self) -> Option<usize> {
        if self.cycles.values().all(|cycle_len| cycle_len.is_some()) {
            Some(
                self.cycles
                    .values()
                    .map(|cycle_len| cycle_len.unwrap())
                    .product(),
            )
        } else {
            None
        }
    }
}

enum Module {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
}

impl Module {
    fn outputs(&self) -> &[u64] {
        match self {
            Self::Broadcaster(broadcaster_module) => &broadcaster_module.outputs,
            Self::FlipFlop(flip_flop_module) => &flip_flop_module.outputs,
            Self::Conjunction(conjunction_module) => &conjunction_module.outputs,
        }
    }

    fn resolve(&mut self, pulse: &Pulse) -> Option<Pulse> {
        match self {
            Self::Broadcaster(broadcaster) => Some(broadcaster.resolve(pulse)),
            Self::FlipFlop(flip_flop) => flip_flop.resolve(pulse),
            Self::Conjunction(conjunction) => Some(conjunction.resolve(pulse)),
        }
    }
}

struct BroadcasterModule {
    id: u64,
    outputs: Vec<u64>,
}

impl BroadcasterModule {
    fn resolve(&self, pulse: &Pulse) -> Pulse {
        Pulse {
            from: self.id,
            high: pulse.high,
        }
    }
}

struct FlipFlopModule {
    id: u64,
    on: bool,
    outputs: Vec<u64>,
}

impl FlipFlopModule {
    fn resolve(&mut self, pulse: &Pulse) -> Option<Pulse> {
        if !pulse.high {
            self.on = !self.on;
            Some(Pulse {
                from: self.id,
                high: self.on,
            })
        } else {
            None
        }
    }
}

struct ConjunctionModule {
    id: u64,
    memory: HashMap<u64, bool>,
    outputs: Vec<u64>,
}

impl ConjunctionModule {
    fn resolve(&mut self, pulse: &Pulse) -> Pulse {
        *self.memory.get_mut(&pulse.from).unwrap() = pulse.high;
        Pulse {
            from: self.id,
            high: !self.memory.values().all(|high| *high),
        }
    }
}

struct Pulse {
    from: u64,
    high: bool,
}

pub fn part_one(input: &str) -> usize {
    let mut state = input.parse::<State>().unwrap();
    for count in 0..1000 {
        state.push_button(count);
    }
    state.low_pulse_count * state.high_pulse_count
}

pub fn part_two(input: &str) -> usize {
    let mut state = input.parse::<State>().unwrap();
    let mut count = 0;
    loop {
        count += 1;
        state.push_button(count);
        if let Some(cycles_to_activate) = state.cycles_to_activate() {
            break cycles_to_activate;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/20/1").unwrap();
        assert_eq!(part_one(&input), 32000000);
        let input = read_to_string("examples/20/2").unwrap();
        assert_eq!(part_one(&input), 11687500);
    }
}
