use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use num::Integer;

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let mut network = input.parse::<Network>().unwrap();

    let untyped_module = &network.modules[network.untyped_index.0];
    assert_eq!(untyped_module.inputs.len(), 1);
    let input_index = untyped_module.inputs[0];
    let input_module = &network.modules[input_index.0];

    let mut check_modules = input_module
        .inputs
        .iter()
        .map(|input| (*input, None))
        .collect();

    for step in 1..=1000 {
        network.press_button(step, &mut check_modules);
    }
    let part1 = network.low_pulse_count * network.high_pulse_count;

    let mut step = 1001;
    loop {
        network.press_button(step, &mut check_modules);

        if check_modules.iter().all(|(_, cycle)| cycle.is_some()) {
            break;
        }

        step += 1;
    }
    let part2 = check_modules
        .into_iter()
        .map(|(_, cycle)| cycle.unwrap())
        .reduce(|a, b| a.lcm(&b))
        .unwrap();

    (part1, part2)
}

#[derive(Debug)]
struct Network {
    modules: Vec<Module>,
    broadcaster_index: ModuleIndex,
    untyped_index: ModuleIndex,
    low_pulse_count: u64,
    high_pulse_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ModuleIndex(usize);

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    inputs: Vec<ModuleIndex>,
    outputs: Vec<ModuleIndex>,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop { on: bool },
    Conjunction { last_pulses: Vec<Pulse> },
    Broadcast,
    Untyped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

impl Network {
    fn press_button(&mut self, step: usize, check_modules: &mut Vec<(ModuleIndex, Option<usize>)>) {
        let mut pulse_queue = VecDeque::new();
        pulse_queue.push_front((self.broadcaster_index, self.broadcaster_index, Pulse::Low));
        while let Some((source, destination, pulse)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::High => self.high_pulse_count += 1,
                Pulse::Low => self.low_pulse_count += 1,
            }

            if !check_modules.is_empty() && pulse == Pulse::High {
                for (check_module, value) in check_modules.iter_mut() {
                    if value.is_some() {
                        continue;
                    }
                    if source == *check_module {
                        *value = Some(step);
                    }
                }
            }

            let module = &mut self.modules[destination.0];
            let result_pulse = match &mut module.module_type {
                ModuleType::FlipFlop { on } => {
                    if pulse == Pulse::Low {
                        *on = !*on;
                        Some(Pulse::from_bool(*on))
                    } else {
                        None
                    }
                }
                ModuleType::Conjunction { last_pulses } => {
                    let last_pulse_index = module
                        .inputs
                        .iter()
                        .enumerate()
                        .find(|(_, other_index)| **other_index == source)
                        .unwrap()
                        .0;
                    last_pulses[last_pulse_index] = pulse;

                    let all_high = last_pulses.iter().all(|pulse| *pulse == Pulse::High);
                    Some(Pulse::from_bool(!all_high))
                }
                ModuleType::Broadcast => Some(pulse),
                ModuleType::Untyped => None,
            };

            if let Some(result_pulse) = result_pulse {
                for output in module.outputs.iter() {
                    pulse_queue.push_back((destination, *output, result_pulse));
                }
            }
        }
    }

    fn _reset(&mut self) {
        for module in self.modules.iter_mut() {
            match &mut module.module_type {
                ModuleType::FlipFlop { on } => {
                    *on = false;
                }
                ModuleType::Conjunction { last_pulses } => {
                    for last_pulse in last_pulses.iter_mut() {
                        *last_pulse = Pulse::Low;
                    }
                }
                ModuleType::Broadcast => {}
                ModuleType::Untyped => {}
            }
        }
    }
}

impl FromStr for Network {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut partials = Vec::new();
        for line in s.lines() {
            let (name, outputs) = line.split_once(" -> ").unwrap();
            let (name, module_type) = if name.starts_with('%') {
                (&name[1..], ModuleType::FlipFlop { on: false })
            } else if name.starts_with('&') {
                (
                    &name[1..],
                    ModuleType::Conjunction {
                        last_pulses: Vec::new(),
                    },
                )
            } else {
                (name, ModuleType::Broadcast)
            };

            partials.push((name, module_type, outputs));
        }

        let module_indexes = partials
            .iter()
            .enumerate()
            .map(|(i, (name, _, _))| (name.to_string(), i))
            .collect::<HashMap<_, _>>();
        let broadcaster_index = ModuleIndex(*module_indexes.get("broadcaster").unwrap());
        let untyped_index = ModuleIndex(partials.len());

        let mut modules = Vec::new();
        for (_, module_type, outputs) in partials {
            modules.push(Module {
                module_type,
                inputs: Vec::new(),
                outputs: outputs
                    .split(", ")
                    .map(|output_name| {
                        module_indexes
                            .get(output_name)
                            .map(|i| ModuleIndex(*i))
                            .unwrap_or(untyped_index)
                    })
                    .collect(),
            });
        }
        modules.push(Module {
            module_type: ModuleType::Untyped,
            inputs: Vec::new(),
            outputs: Vec::new(),
        });

        for i in 0..modules.len() {
            let module = &modules[i];
            let output_indexes = module
                .outputs
                .iter()
                .map(|output| output.0)
                .collect::<Vec<_>>();
            for output_index in output_indexes {
                modules[output_index].inputs.push(ModuleIndex(i));
            }
        }

        for module in modules.iter_mut() {
            if let ModuleType::Conjunction { last_pulses } = &mut module.module_type {
                *last_pulses = vec![Pulse::Low; module.inputs.len()];
            }
        }

        Ok(Network {
            modules,
            broadcaster_index,
            untyped_index,
            low_pulse_count: 0,
            high_pulse_count: 0,
        })
    }
}

impl Pulse {
    fn from_bool(value: bool) -> Pulse {
        match value {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        let (part1, _) = solve(input);

        assert_eq!(part1.to_string(), "32000000");
    }
}
