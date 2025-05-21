// Day 20 of Advent of Code, 2023
use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs,
    path::PathBuf,
};

use num::integer::lcm;

enum ModuleType<'a> {
    FlipFlop {
        state: bool,
    },
    Conjunction {
        input_states: HashMap<&'a str, bool>,
    },
    Broadcaster,
}

struct Module<'a> {
    module_type: ModuleType<'a>,
    outputs: Vec<&'a str>,
}

fn parse_line(line: &str) -> (&str, Module) {
    let (address, destination_list) = match line.split_once(" -> ") {
        Some(parts) => parts,
        None => panic!("Invalid line format: {}", line),
    };
    let outputs = destination_list.split(", ").collect::<Vec<&str>>();

    let first_char = match address.chars().next() {
        Some(c) => c,
        None => panic!("Invalid address: {}", address),
    };

    match first_char {
        'b' => (
            address,
            Module {
                module_type: ModuleType::Broadcaster,
                outputs: outputs,
            },
        ),
        '%' => (
            &address[1..],
            Module {
                module_type: ModuleType::FlipFlop { state: false },
                outputs: outputs,
            },
        ),
        '&' => (
            &address[1..],
            Module {
                module_type: ModuleType::Conjunction {
                    input_states: HashMap::new(),
                },
                outputs: outputs,
            },
        ),
        _ => panic!("Unknown module type"),
    }
}

fn process_module<'a>(
    source: &'a str,
    dest: &'a str,
    pulse: bool,
    modules: &mut HashMap<&'a str, Module<'a>>,
    message_queue: &mut VecDeque<(&'a str, &'a str, bool)>,
) {
    if let Some(dest_module) = modules.get_mut(dest) {
        match &mut dest_module.module_type {
            ModuleType::FlipFlop { state } => {
                if !pulse {
                    *state = !*state;
                    for output in &dest_module.outputs {
                        message_queue.push_back((dest, output, *state));
                    }
                }
            }
            ModuleType::Conjunction { input_states } => {
                input_states.insert(source, pulse);
                let message = !input_states.values().all(|v| *v);
                for output in &dest_module.outputs {
                    message_queue.push_back((dest, output, message));
                }
            }
            ModuleType::Broadcaster => {
                for output in &dest_module.outputs {
                    message_queue.push_back((dest, output, pulse));
                }
            }
        }
    }
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let mut modules = puzzle_input
        .lines()
        .map(|l| parse_line(l))
        .collect::<HashMap<_, _>>();

    let connections = modules
        .iter()
        .flat_map(|(address, module)| module.outputs.iter().map(move |output| (*address, *output)))
        .collect::<Vec<_>>();

    for (source, dest) in connections.iter() {
        if let Some(dest_module) = modules.get_mut(dest) {
            if let ModuleType::Conjunction { input_states } = &mut dest_module.module_type {
                input_states.insert(source, false);
            }
        }
    }

    let mut message_queue = VecDeque::new();

    if part_two {
        // Find the connection to the rx input module
        let rx_source = match connections.iter().find(|(_, dest)| *dest == "rx") {
            Some((source, _)) => *source,
            None => panic!("rx module not found!"),
        };

        // Find the final modules of each subgraph outputting to the module before rx
        let mut subgraph_sources = match modules.get(rx_source) {
            Some(Module {
                module_type: ModuleType::Conjunction { input_states },
                ..
            }) => input_states.keys().cloned().collect::<HashSet<_>>(),
            _ => panic!("rx source is not a conjunction module!"),
        };

        let mut button_press_count: i64 = 0;

        let mut subgraph_cycle_lengths = Vec::new();

        while subgraph_sources.len() > 0 {
            message_queue.push_back(("button", "broadcaster", false));
            button_press_count += 1;

            while let Some((source, dest, pulse)) = message_queue.pop_front() {
                if (dest == rx_source) && (subgraph_sources.contains(source)) && pulse {
                    subgraph_sources.remove(source);
                    subgraph_cycle_lengths.push(button_press_count);
                }

                process_module(source, dest, pulse, &mut modules, &mut message_queue);
            }
        }

        // Find the lowest common multiple of the cycle lengths
        subgraph_cycle_lengths.iter().fold(1, |a, b| lcm(a, *b))
    } else {
        let mut high_count = 0;
        let mut low_count = 0;

        for _i in 0..1000 {
            message_queue.push_back(("button", "broadcaster", false));

            while let Some((source, dest, pulse)) = message_queue.pop_front() {
                match pulse {
                    false => low_count += 1,
                    true => high_count += 1,
                }

                process_module(source, dest, pulse, &mut modules, &mut message_queue);
            }
        }

        high_count * low_count
    }
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
