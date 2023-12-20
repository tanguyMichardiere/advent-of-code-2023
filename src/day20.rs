use std::collections::HashMap;
use std::iter;
use std::str::FromStr;

use crate::regex;

struct State {
    modules: HashMap<String, Module>,
    low_pulse_count: usize,
    high_pulse_count: usize,
    before_rx: String,
    cycle_lens: HashMap<String, Option<usize>>,
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
                        .map(|caps| caps["name"].to_owned())
                        .collect();
                    match &caps["name"] {
                        "broadcaster" => (
                            "broadcaster".to_owned(),
                            Module::Broadcaster(BroadcasterModule { outputs }),
                        ),
                        name if name.starts_with("%") => (
                            name[1..].to_owned(),
                            Module::FlipFlop(FlipFlopModule { on: false, outputs }),
                        ),
                        name if name.starts_with("&") => (
                            name[1..].to_owned(),
                            Module::Conjunction(ConjunctionModule {
                                memory: HashMap::new(),
                                outputs,
                            }),
                        ),
                        _ => unreachable!(),
                    }
                })
                .collect::<HashMap<_, _>>();
        let mut empty_modules = Vec::new();
        let mut conjunction_inputs = HashMap::new();
        for (id, module) in &modules {
            for output in module.outputs() {
                match modules.get(output) {
                    None => {
                        empty_modules.push(output.clone());
                    }
                    Some(Module::Conjunction(_)) => {
                        conjunction_inputs
                            .entry(output.clone())
                            .or_insert(Vec::new())
                            .push(id.clone());
                    }
                    _ => {}
                }
            }
        }
        for empty_module in empty_modules {
            modules.insert(empty_module, Module::Empty);
        }
        for (name, inputs) in conjunction_inputs {
            if let Module::Conjunction(conjunction) = modules.get_mut(&name).unwrap() {
                for input in inputs {
                    conjunction.memory.insert(input.clone(), false);
                }
            }
        }
        let before_rx = modules
            .iter()
            .find_map(|(id, module)| {
                if module.outputs().any(|output| output == "rx") {
                    Some(id)
                } else {
                    None
                }
            })
            .unwrap()
            .clone();
        let cycle_lens = modules
            .keys()
            .filter_map(|id| {
                if modules[id].outputs().any(|output| output == &before_rx) {
                    Some((id.clone(), None))
                } else {
                    None
                }
            })
            .collect::<HashMap<_, _>>();
        Ok(Self {
            modules,
            low_pulse_count: 0,
            high_pulse_count: 0,
            before_rx,
            cycle_lens,
        })
    }
}

impl State {
    fn resolve(&mut self, pulses: impl Iterator<Item = Pulse>, count: usize) {
        let mut next_pulses = Vec::new();
        for pulse in pulses {
            if pulse.high {
                self.high_pulse_count += 1;
            } else {
                self.low_pulse_count += 1;
            }
            if pulse.to == self.before_rx && pulse.high {
                *self.cycle_lens.get_mut(&pulse.from).unwrap() = Some(count);
            }
            next_pulses.extend(self.modules.get_mut(&pulse.to).unwrap().resolve(&pulse));
        }
        if !next_pulses.is_empty() {
            self.resolve(next_pulses.into_iter(), count);
        }
    }

    fn push_button(&mut self, count: usize) {
        self.resolve(
            iter::once(Pulse {
                from: "button".to_owned(),
                to: "broadcaster".to_owned(),
                high: false,
            }),
            count,
        );
    }
}

enum Module {
    Broadcaster(BroadcasterModule),
    FlipFlop(FlipFlopModule),
    Conjunction(ConjunctionModule),
    Empty,
}

impl Module {
    fn outputs<'a>(&'a self) -> Box<dyn Iterator<Item = &'a String> + 'a> {
        match self {
            Self::Broadcaster(broadcaster_module) => Box::new(broadcaster_module.outputs.iter()),
            Self::FlipFlop(flip_flop_module) => Box::new(flip_flop_module.outputs.iter()),
            Self::Conjunction(conjunction_module) => Box::new(conjunction_module.outputs.iter()),
            Self::Empty => Box::new(iter::empty()),
        }
    }

    fn resolve<'a>(&'a mut self, pulse: &'a Pulse) -> Box<dyn Iterator<Item = Pulse> + '_> {
        match self {
            Self::Broadcaster(broadcaster) => Box::new(broadcaster.resolve(pulse)),
            Self::FlipFlop(flip_flop) => Box::new(flip_flop.resolve(pulse)),
            Self::Conjunction(conjunction) => Box::new(conjunction.resolve(pulse)),
            Self::Empty => Box::new(iter::empty()),
        }
    }
}

struct BroadcasterModule {
    outputs: Vec<String>,
}

impl BroadcasterModule {
    fn resolve<'a>(&'a self, pulse: &'a Pulse) -> impl Iterator<Item = Pulse> + '_ {
        self.outputs.iter().map(move |output| Pulse {
            from: pulse.to.clone(),
            to: output.to_owned(),
            high: pulse.high,
        })
    }
}

struct FlipFlopModule {
    on: bool,
    outputs: Vec<String>,
}

impl FlipFlopModule {
    fn resolve<'a>(&'a mut self, pulse: &'a Pulse) -> Box<dyn Iterator<Item = Pulse> + '_> {
        if !pulse.high {
            self.on = !self.on;
            Box::new(self.outputs.iter().map(|output| Pulse {
                from: pulse.to.clone(),
                to: output.to_owned(),
                high: self.on,
            }))
        } else {
            Box::new(iter::empty())
        }
    }
}

struct ConjunctionModule {
    memory: HashMap<String, bool>,
    outputs: Vec<String>,
}

impl ConjunctionModule {
    fn resolve<'a>(&'a mut self, pulse: &'a Pulse) -> impl Iterator<Item = Pulse> + '_ {
        *self.memory.get_mut(&pulse.from).unwrap() = pulse.high;
        let high = !self.memory.values().all(|high| *high);
        self.outputs.iter().map(move |output| Pulse {
            from: pulse.to.clone(),
            to: output.to_owned(),
            high,
        })
    }
}

struct Pulse {
    from: String,
    to: String,
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
        if state
            .cycle_lens
            .values()
            .all(|cycle_len| cycle_len.is_some())
        {
            break state
                .cycle_lens
                .into_values()
                .map(|cycle_len| cycle_len.unwrap())
                .product();
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
