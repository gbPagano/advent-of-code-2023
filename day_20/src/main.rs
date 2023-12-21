use num_integer::lcm;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // part_one(contents);
    part_two(contents);
}

fn part_one(contents: String) {
    let mut modules = contents
        .split("\n")
        .map(|line| {
            let (module, dest) = line.split_once(" -> ").unwrap_or_else(|| unreachable!());
            match module.chars().nth(0).unwrap() {
                '%' => (
                    module[1..].to_string(),
                    Module::new_flip_flop(
                        module[1..].to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
                '&' => (
                    module[1..].to_string(),
                    Module::new_conjunction(
                        module[1..].to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
                _ => (
                    module.to_string(),
                    Module::new_broadcast(
                        module.to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
            }
        })
        .collect::<HashMap<String, Module>>();

    {
        let name_dest: HashMap<String, Vec<String>> = modules
            .iter()
            .map(|(name, module)| (name.clone(), module.destinations.clone()))
            .collect();

        name_dest.iter().for_each(|(name, destinations)| {
            destinations.iter().for_each(|dest| {
                if let Some(module) = modules.get_mut(dest) {
                    match module.class {
                        ModuleType::Conjunction(ref mut conj) => {
                            conj.inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => (),
                    }
                }
            })
        });
    }

    let mut pulses_count: HashMap<Pulse, usize> = HashMap::new();
    pulses_count.insert(Pulse::Low, 0);
    pulses_count.insert(Pulse::High, 0);
    for _ in 0..1000 {
        pulses_count.insert(Pulse::Low, pulses_count[&Pulse::Low] + 1);

        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        modules
            .get("broadcaster")
            .unwrap()
            .destinations
            .iter()
            .for_each(|dest| {
                queue.push_back(("broadcaster".to_string(), Pulse::Low, dest.clone()));
                pulses_count.insert(Pulse::Low, pulses_count[&Pulse::Low] + 1);
            });

        while let Some((sender, value, dest)) = queue.pop_front() {
            // dbg!((sender.clone(), value, dest.clone()));
            if let Some(module) = modules.get_mut(&dest) {
                if let Some(next_pulses) = module.send_pulse(sender, value) {
                    next_pulses.into_iter().for_each(|(value, dest)| {
                        queue.push_back((module.name.clone(), value, dest.clone()));
                        pulses_count.insert(value, pulses_count[&value] + 1);
                    });
                }
            }
        }
    }

    let res = pulses_count[&Pulse::Low] * pulses_count[&Pulse::High];
    println!("res: {res}");
}

fn part_two(contents: String) {
    let mut modules = contents
        .split("\n")
        .map(|line| {
            let (module, dest) = line.split_once(" -> ").unwrap_or_else(|| unreachable!());
            match module.chars().nth(0).unwrap() {
                '%' => (
                    module[1..].to_string(),
                    Module::new_flip_flop(
                        module[1..].to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
                '&' => (
                    module[1..].to_string(),
                    Module::new_conjunction(
                        module[1..].to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
                _ => (
                    module.to_string(),
                    Module::new_broadcast(
                        module.to_string(),
                        dest.split(", ").map(String::from).collect::<Vec<String>>(),
                    ),
                ),
            }
        })
        .collect::<HashMap<String, Module>>();

    let mut modules_to_eval: Vec<String> = Vec::new();
    let mut conj_rx = String::new();
    {
        let name_dest: HashMap<String, Vec<String>> = modules
            .iter()
            .map(|(name, module)| (name.clone(), module.destinations.clone()))
            .collect();

        name_dest.iter().for_each(|(name, destinations)| {
            destinations.iter().for_each(|dest| {
                if let Some(module) = modules.get_mut(dest) {
                    match module.class {
                        ModuleType::Conjunction(ref mut conj) => {
                            conj.inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => (),
                    }
                } else if dest == "rx" {
                    conj_rx = name.clone();
                }
            })
        });

        match modules.get_mut(&conj_rx).unwrap().class {
            ModuleType::Conjunction(ref mut conj) => {
                let keys: Vec<String> = conj.inputs.keys().map(|s| s.to_string()).collect();
                modules_to_eval.extend(keys);
            }
            _ => (),
        }
    }

    let mut cycle_lenghts: HashMap<String, usize> =
        modules_to_eval.iter().map(|s| (s.to_string(), 0)).collect();
    let mut seen = cycle_lenghts.clone();
        
    let mut button_clicks = 0;
    loop {
        button_clicks += 1;
        let mut queue: VecDeque<(String, Pulse, String)> = VecDeque::new();
        modules
            .get("broadcaster")
            .unwrap()
            .destinations
            .iter()
            .for_each(|dest| {
                queue.push_back(("broadcaster".to_string(), Pulse::Low, dest.clone()));
            });

        while let Some((sender, value, dest)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&dest) {
                if let Some(next_pulses) = module.send_pulse(sender, value) {
                    for (value, dest) in next_pulses.into_iter() {
                        queue.push_back((module.name.clone(), value, dest.clone()));
                        let key = module.name.clone();
                        if cycle_lenghts.contains_key(&key) && dest == conj_rx && value == Pulse::High {
                            seen.insert(key.clone(), seen[&key] + 1);
                            cycle_lenghts.insert(key.clone(), button_clicks);
                        }
                    }
                }
            }
        }
        if seen.iter().all(|(_,v)| *v > 0) {
            dbg!(&seen);
            break;
        }
    }
    dbg!(&cycle_lenghts);

    let res = cycle_lenghts.values().cloned().fold(1, lcm);
    println!("res: {:?}", res);
}

#[derive(Debug)]
struct Module {
    name: String,
    class: ModuleType,
    destinations: Vec<String>,
}

impl Module {
    fn new_flip_flop(name: String, destinations: Vec<String>) -> Module {
        Module {
            name,
            class: ModuleType::FlipFlop(FlipFlop { state: State::Off }),
            destinations,
        }
    }

    fn new_conjunction(name: String, destinations: Vec<String>) -> Module {
        Module {
            name,
            class: ModuleType::Conjunction(Conjunction {
                inputs: HashMap::new(),
            }),
            destinations,
        }
    }

    fn new_broadcast(name: String, destinations: Vec<String>) -> Module {
        Module {
            name,
            class: ModuleType::Broadcast,
            destinations,
        }
    }

    fn send_pulse(&mut self, sender: String, value: Pulse) -> Option<Vec<(Pulse, String)>> {
        match (&mut self.class, value) {
            (&mut ModuleType::FlipFlop(ref mut flip_flop), Pulse::Low) => match &flip_flop.state {
                &State::Off => {
                    flip_flop.state = State::On;
                    Some(
                        self.destinations
                            .iter()
                            .map(|dest| (Pulse::High, dest.clone()))
                            .collect::<Vec<_>>(),
                    )
                }
                &State::On => {
                    flip_flop.state = State::Off;
                    Some(
                        self.destinations
                            .iter()
                            .map(|dest| (Pulse::Low, dest.clone()))
                            .collect::<Vec<_>>(),
                    )
                }
            },
            (&mut ModuleType::Conjunction(ref mut conjunction), _) => {
                conjunction.inputs.insert(sender.clone(), value);
                if conjunction
                    .inputs
                    .iter()
                    .all(|(_, val)| *val == Pulse::High)
                {
                    Some(
                        self.destinations
                            .iter()
                            .map(|dest| (Pulse::Low, dest.clone()))
                            .collect::<Vec<_>>(),
                    )
                } else {
                    Some(
                        self.destinations
                            .iter()
                            .map(|dest| (Pulse::High, dest.clone()))
                            .collect::<Vec<_>>(),
                    )
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    state: State,
}

#[derive(Debug)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast,
}
